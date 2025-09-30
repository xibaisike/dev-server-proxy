````markdown
# Feature Specification: 类 nginx 的网站代理服务（feat-nginx-proxy）

**Feature Branch**: `feat/nginx-proxy`  
**Created**: 2025-09-30  
**Status**: Draft  
**Input**: User description: "实现一个类nginx的网站代理服务，通过配置文件(后缀.njs) 告知dev-server-proxy 如何代理本地请求到远端服务器。 - 在README.md中完善njs配置文件的说明文档，用户阅读之后能够独立编写配置文件 - 在packages/app/src中实现配置文件解析，并实现其功能 - 在cli tools中添加工具 1. help 2. 日志功能：输出日志文件，包含请求事件，请求path, 代理path， http status等"

## Execution Flow (main)
```
1. Parse user description from Input
   → If empty: ERROR "No feature description provided"
2. Create feature branch and initialize spec file (script .specify helper normally does this)
   → If helper script cannot run: create spec file manually and record branch name (assumption)
3. Define user scenarios & acceptance tests
4. Produce functional requirements (each testable)
5. Design minimal data shapes and runtime behavior
6. Implement TDD: write Jest tests for core behaviors (config parsing, rule matching, pathRewrite, proxying, logging)
7. Implement proxy server and CLI
8. Run smoke tests and a minimal end-to-end test
9. Iterate until tests green
```

---

## ⚡ Quick Guidelines
- This spec focuses on end-user behavior and testable requirements.
- Implementation notes (typescript, esbuild, jest, pnpm) are included as developer guidance but the acceptance criteria are behavior and tests.

## User Scenarios & Testing *(mandatory)*

### Primary User Story
作为前端开发者，我希望在本地启动 dev-server-proxy 并通过一个 `.njs` 配置文件指定路由匹配规则与目标服务器，这样浏览器对我的开发域名的请求会被透明代理到远端或本地后端，且必要时可注入 HTML、重写路径或代理 websocket。

### Acceptance Scenarios
1. Given 已有一个 `proxy.config.njs` 并包含 locations 规则，When 启动 `dsp ./proxy.config.njs`，Then 所有匹配到的请求应按规则代理到对应的 `target`，并返回来自目标的 HTTP 响应（200/3xx/4xx/5xx）。
2. Given location 含 `pathRewrite`，When 请求到匹配路径，Then 转发到目标时应先对请求路径执行重写（按键对应正则替换）。
3. Given location 含 `inject` 并且目标返回 HTML，When 请求根路径（`=` 精确匹配或 `/` 前缀匹配到 HTML），Then 在响应体的适当位置注入 `inject` 字符串并返回给客户端（保持 Content-Type）。
4. Given websocket 配置，When 客户端建立 websocket 连接到匹配路径，Then 代理应将 websocket 协议转发到 `websocket.target`。
5. Given 任何代理请求完成，When 请求结束，Then 记录日志到 `proxy.log`，包含 timestamp、事件（proxy/no-match/bad-request）、原始请求 path、最终代理目标（含被重写的 path）、HTTP status。

### Edge Cases
- 配置文件无效（语法错误或缺少 required 字段）：代理应拒绝启动并打印错误信息。
- pathRewrite 匹配无效正则：应忽略该 rewrite 并记录警告（不要崩溃）。
- 当目标不可达或超时：应将错误码与消息返回给客户端，并在日志中记录目标地址与错误信息。
- 注入在非 HTML 响应上：不得注入，记录事件并继续返回原始响应。
- 并发高请求量：代理应保持稳定（基本容错即可，超大规模不在本次范围）。

## Requirements *(mandatory)*

### Functional Requirements
- **FR-001**: 系统 MUST 能从传入的 `.njs` ESM 模块中加载配置对象（`export default { ... }`）。
- **FR-002**: 系统 MUST 支持 nginx-like `locations` 匹配语法：`=`, `^~`, `~`, `~*`, `/`，并按指定优先级匹配。
- **FR-003**: 系统 MUST 在匹配到 `location` 时将请求代理到 `target`，并支持对 `req.url` 的 `pathRewrite` 转换。
- **FR-004**: 系统 MUST 支持 `inject` 字段以在 HTML 响应中注入字符串（仅当 Content-Type 为 HTML 时）。
- **FR-005**: 系统 MUST 支持可选的 `websocket.target`，并代理 websocket 连接。
- **FR-006**: 系统 MUST 记录日志到运行目录下的 `proxy.log`，每项日志包含 ISO timestamp、事件、原始请求 path、代理目标（含重写后的 path）、HTTP status。
- **FR-007**: 系统 MUST 在启动时输出加载的配置路径与监听端口，并在无法解析配置时退出且返回非零码。
- **FR-008**: 系统 MUST 在无法匹配任何 location 时返回 502 并记录 `no-match` 日志事件。

*Notes/Non-functional*: 首选 TypeScript + ESM，使用 `http-proxy` 作为代理实现；TDD（Jest）覆盖关键函数：配置加载、规则匹配、path rewrite、logging。

### Key Entities
- **Config**: { server_name?, defaultTarget, websocket?, locations: Location[] }
- **Location**: { rule: string, target: string, pathRewrite?: Record<string,string>, inject?: string }
- **LogEntry**: { timestamp: ISOString, event: 'proxy'|'no-match'|'bad-request'|'error', reqPath: string, proxyPath: string, status: number }

## Review & Acceptance Checklist

### Content Quality
- [x] No higher-level implementation uncertainties remain regarding the feature scope
- [x] Behavior is described and testable

### Requirement Completeness
- [ ] No [NEEDS CLARIFICATION] markers remain (see below for clarifications)
- [x] Requirements are testable and unambiguous

## [NEEDS CLARIFICATION]
- 启动默认监听端口应为哪个：spec 假定若配置对象中有 `port` 字段则使用该端口，否则使用 80；是否接受此行为？
- 日志轮转/大小限制是否需要内建支持？当前实现只追加到 `proxy.log`，运维上可能需要 logrotate 配合。

## Execution Status
- [x] User description parsed
- [x] Key concepts extracted
- [x] Ambiguities marked
- [x] User scenarios defined
- [x] Requirements generated
- [x] Entities identified
- [ ] Review checklist passed

---

### Next steps (developer tasks)
1. Add Jest unit tests in `packages/app` for:
   - config loader (loads ESM `.njs` files reliably)
   - `getLocationMatch` function (test all rule types and priority)
   - `applyPathRewrite` (regex-based rewrites)
   - logging (append to `proxy.log` and correct format)
2. Implement (or refine) `packages/app/src/services/proxy-server.ts` to satisfy FR-001..FR-008.
3. Add `packages/app/cli.js` with `help` and `log` subcommands and validate execution flow.
4. Run smoke test: start server with example `packages/app/proxy.config.njs`, send sample requests (HTTP + websocket), assert behavior and logs.

````
