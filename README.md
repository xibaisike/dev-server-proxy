# dev-server-proxy

## Proxy for web development  

Proxying some URLs can be useful when you have a separate API backend development server and you want to send API requests on the same domain.

With a backend on https://copilot.ai, you can use this to enable proxying:

```javascript
export default {
  server_name: 'https://dev-copilot.ai',
  defaultTarget: 'http://localhost:8080'
}
```
A request to http://localhost:8080/api/users will now proxy the request to https://copilot.ai/api/users


If you don't want /api to be passed along, we need to rewrite the path:

```javascript
const DEV_HOST = 'http://localhost:8080';
export default {
  server_name: 'https://dev-copilot.ai',
  defaultTarget: DEV_HOST,
  locations: [
    {
      rule: '^~ /api',
      target: DEV_HOST,
      pathRewrite: { '^/api': '' },
    }
  ]
}
```

## 配置文件说明（.njs）

dev-server-proxy 通过 njs 配置文件（推荐命名为 `proxy.config.njs`）实现灵活的本地到远端代理，支持类似 nginx 的 location 匹配规则。

### 配置文件结构

```javascript
export default {
  /**
   * server_name: 虚拟服务域名，仅用于标识
   */
  server_name: 'https://dev-pai.test.com',

  /**
   * defaultTarget: 默认代理目标，所有未被 locations 匹配的请求都将代理到此
   */
  defaultTarget: 'http://localhost:8080',

  /**
   * websocket: 可选，websocket 代理目标
   */
  websocket: {
    target: 'wss://ws.example.com'
  },

  /**
   * locations: 路由规则数组，按顺序匹配
   */
  locations: [
    {
      rule: '= /', // 精确匹配
      target: 'http://localhost:8080/index.html',
      inject: `\n<script>window.globalConfig = {base: "http://localhost:8888/"}</script>\n`, // 可选，注入 HTML 片段
    },
    {
      rule: '^~ /api', // 前缀匹配
      target: 'http://localhost:8080',
      pathRewrite: { '^/api': '' }, // 可选，路径重写
    },
    {
      rule: '~ \\.ipynb$', // 正则匹配（区分大小写）
      target: 'http://localhost:8080',
      pathRewrite: { '^/.*$': '/index.html' },
    },
    {
      rule: '~* \\.(css|js)$', // 正则匹配（不区分大小写）
      target: 'http://localhost:8080',
    },
    {
      rule: '/', // 默认前缀
      target: 'http://localhost:8080',
    }
  ]
}
```

### 规则说明

- `= /path`：精确匹配
- `^~ /prefix`：前缀匹配
- `~ regex`：正则匹配（区分大小写）
- `~* regex`：正则匹配（不区分大小写）
- `/`：默认前缀匹配

location 匹配按顺序，命中即停止。

#### 字段说明

- `rule`：匹配规则，支持上述 nginx-like 语法
- `target`：代理目标地址
- `inject`：可选，若为 HTML 请求可注入脚本片段
- `pathRewrite`：可选，路径重写规则，键为正则表达式，值为替换字符串

### 完整示例

```javascript
const DEV_HOST = 'http://localhost:8080';
const API_HOST = 'https://api.example.com';
export default {
  server_name: 'https://dev.example.com',
  defaultTarget: DEV_HOST,
  websocket: {
    target: 'wss://ws.example.com'
  },
  locations: [
    {
      rule: '= /',
      target: `${DEV_HOST}/index.html`,
      inject: `\n<script>window.globalConfig = {base: "http://localhost:8888/"}</script>\n`,
    },
    {
      rule: '^~ /api',
      target: API_HOST,
      pathRewrite: { '^/api': '' },
    },
    {
      rule: '~* \\.(css|js|png)$',
      target: DEV_HOST,
    },
    {
      rule: '/',
      target: DEV_HOST
    }
  ]
}
```

### 使用方法

1. 在项目根目录创建 `proxy.config.njs`，内容参考上方示例。
2. 启动代理服务：

   ```sh
   dsp ./proxy.config.njs
   ```

3. 访问本地服务，所有请求将按配置自动代理。

---
...existing code...


## Install & Launch

```
npm i -g @dorado/dev-server-proxy
```

```
dsp ./proxy.njs
```

