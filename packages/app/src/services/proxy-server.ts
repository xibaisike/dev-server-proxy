import http from "http";
import httpProxy from "http-proxy";
import { parse } from "url";
import path from "path";
import { pathToFileURL } from "url";
import fs from "fs";
const logPath = path.resolve(process.cwd(), 'proxy.log');
export type LogEvent = { event: string; reqPath: string; proxyPath: string; status: number };
export function logEvent({ event, reqPath, proxyPath, status }: LogEvent) {
  const line = `[${new Date().toISOString()}] ${event} | req: ${reqPath} | proxy: ${proxyPath} | status: ${status}\n`;
  fs.appendFile(logPath, line, () => {});
}

// 获取配置文件路径
const configPath = process.argv[2] || "./proxy.config.njs";

export async function loadConfigFromPath(p: string) {
  const configUrl = pathToFileURL(path.resolve(p)).href;
  const mod = await import(configUrl);
  return mod.default;
}

async function loadConfig() {
  return loadConfigFromPath(configPath);
}

// nginx location 匹配优先级实现
export function getLocationMatch(locations: any[], pathname: string) {
  // 1. 精确匹配 =
  for (const loc of locations) {
    if (loc.rule && loc.rule.startsWith('= ')) {
      const exact = loc.rule.slice(2).trim();
      if (pathname === exact) return loc;
    }
  }
  // 2. 前缀 ^~
  let bestPrefix = null;
  for (const loc of locations) {
    if (loc.rule && loc.rule.startsWith('^~ ')) {
      const prefix = loc.rule.slice(3).trim();
      if (pathname.startsWith(prefix)) {
        if (!bestPrefix || prefix.length > bestPrefix.prefix.length) {
          bestPrefix = { ...loc, prefix };
        }
      }
    }
  }
  if (bestPrefix) return bestPrefix;
  // 3. 正则 ~
  for (const loc of locations) {
    if (loc.rule && loc.rule.startsWith('~ ')) {
      const regexStr = loc.rule.slice(2).trim();
      const regex = new RegExp(regexStr);
      if (regex.test(pathname)) return loc;
    }
    if (loc.rule && loc.rule.startsWith('~* ')) {
      const regexStr = loc.rule.slice(3).trim();
      const regex = new RegExp(regexStr, 'i');
      if (regex.test(pathname)) return loc;
    }
  }
  // 4. 普通前缀 /
  let bestSlash = null;
  for (const loc of locations) {
    if (loc.rule && loc.rule === '/') {
      if (!bestSlash) bestSlash = loc;
    } else if (loc.rule && pathname.startsWith(loc.rule)) {
      if (!bestSlash || loc.rule.length > bestSlash.rule.length) {
        bestSlash = loc;
      }
    }
  }
  if (bestSlash) return bestSlash;
  return null;
}

export function applyPathRewrite(pathname: string, pathRewrite?: Record<string, string>) {
  if (!pathRewrite) return pathname;
  let newPath = pathname;
  for (const [pattern, replacement] of Object.entries(pathRewrite)) {
    try {
      newPath = newPath.replace(new RegExp(pattern), replacement);
    } catch (e) {
      // ignore invalid regex
    }
  }
  return newPath;
}

async function startProxy() {
  const config = await loadConfig();
  const proxy = httpProxy.createProxyServer({ changeOrigin: true });
  const port = config.port || 80;
  const locations = config.locations || [];

  const server = http.createServer((req, res) => {
    const { pathname = "" } = parse(req.url || "");
    if (!pathname) {
      res.writeHead(400);
      res.end("Bad Request");
      logEvent({ event: 'bad-request', reqPath: req.url || '', proxyPath: '', status: 400 });
      return;
    }
    const loc = getLocationMatch(locations, pathname);
    if (loc && loc.target) {
      // pathRewrite
      let proxyPath = pathname;
      if (loc.pathRewrite) {
        proxyPath = applyPathRewrite(proxyPath, loc.pathRewrite);
        // 替换 req.url
        const urlObj = parse(req.url || "");
        req.url = proxyPath + (urlObj.search || "");
      }
      proxy.web(req, res, { target: loc.target });
      res.on('finish', () => {
        logEvent({ event: 'proxy', reqPath: pathname, proxyPath: loc.target + req.url, status: res.statusCode });
      });
      return;
    }
    res.writeHead(502);
    res.end("No proxy target matched");
    logEvent({ event: 'no-match', reqPath: req.url || '', proxyPath: '', status: 502 });
  });

  server.listen(port, () => {
    console.log(`Proxy server listening on port ${port}`);
    console.log(`Loaded config: ${configPath}`);
  });
}

startProxy();
