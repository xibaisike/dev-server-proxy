#!/usr/bin/env node
import { spawn } from 'child_process';
import path from 'path';

const args = process.argv.slice(2);
const [command, ...rest] = args;

function printHelp() {
  console.log(`dev-server-proxy CLI

Usage:
  dsp <config.njs>         启动代理服务
  dsp help                 显示帮助信息
  dsp log                  查看日志文件
`);
}

function showLog() {
  const logPath = path.resolve(process.cwd(), 'proxy.log');
  console.log(`日志文件: ${logPath}\n`);
  spawn('cat', [logPath], { stdio: 'inherit' });
}

if (command === 'help' || !command) {
  printHelp();
  process.exit(0);
}
if (command === 'log') {
  showLog();
  process.exit(0);
}
// 默认启动代理
const configFile = command.endsWith('.njs') ? command : 'proxy.config.njs';
const proxyServer = path.resolve(__dirname, 'src/services/proxy-server.ts');
const child = spawn('tsx', [proxyServer, configFile, ...rest], { stdio: 'inherit' });
child.on('exit', code => process.exit(code ?? 0));
