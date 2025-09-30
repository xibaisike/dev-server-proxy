// proxy.config.njs
// ESM config for proxy server, flexible route config
const PAIURL = 'https://pai.console.aliyun.com';
const DEV_HOST = 'http://127.0.0.1:9102';
const JUPYTER_HOST = 'http://127.0.0.1:8888';

// Nginx-like location rules
// 支持：
//   - = 精确匹配
//   - ^~ 前缀匹配
//   - ~ 正则匹配（区分大小写）
//   - ~* 正则匹配（不区分大小写）
//   - / 默认前缀

export default {
  /**
   * 虚拟server域名
   */
  server_name: 'https://dev-pai.test.com',
  // 默认所有静态资源(css|js|html|png)被代理到target
  defaultTarget: DEV_HOST,
  /**
   * 代理websocket server
   */
  websocket: {
    target: PAIURL
  },
  locations: [
    {
      rule: '= /',
      target: `${DEV_HOST}/index.html`,
      inject: `
        <script>window.globalConfig = {base: "http://localhost:8888/"} </script>
      `,
    },
    {
      rule: '= /data/api.json',
      target: `${PAIURL}/data/api.json`,
    },
    {
      rule: '^~ /lab',
      target: `${DEV_HOST}/index.html`,
    },
    {
      rule: '~ \\.ipynb$',
      target: DEV_HOST,
      pathRewrite: { '^/.*$': '/index.html' },
    },
    {
      rule: '~ (/lab)?/api',
      target: JUPYTER_HOST,
    },
    {
      rule: '~* \\.(css|js)$',
      target: DEV_HOST,
    },
    {
      rule: '/',
      target: DEV_HOST
    }
  ]
};
