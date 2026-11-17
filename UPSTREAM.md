# 上游来源与授权声明

本仓库是 **EAiCoding 0.1.10** 的本地构建版本。

## 原作者与上游仓库

- **原作者**：ordinarykeys
- **上游仓库**：https://github.com/ordinarykeys/eaicoding
- **本仓库基于的上游提交**：`1db018c6ce8f206c735b0cc62540a7538c6082d7`（ix: isolate task runs and align Harness queue in v0.1.10）
- **上游版本**：0.1.10

## 授权条款（引自上游 README「授权说明」）

本项目采用源码公开、**非商业授权**的方式发布。源码仅限学习、交流、研究和个人非商业用途。

未经作者书面授权，禁止将本项目或基于本项目的修改版本用于以下用途：

- 商业销售、付费分发、软件代售或二次打包售卖。
- 作为商业产品、商业 SaaS、付费会员服务的一部分提供给第三方使用。
- 用于商业培训、商业课程、商业外包交付。
- 去除作者信息后重新发布。
- 使用本项目名称、图标、界面或说明材料进行商业宣传。

允许的用途：

- 个人学习和研究。
- 非商业项目中的测试和改造。
- 提交 issue、PR 或基于源码进行技术交流。
- 在保留作者信息和本说明的前提下进行非商业分享。

第三方依赖、易语言相关工具链、模型文件和其他外部资源分别遵循其自身许可证或授权条款。本项目的非商业限制不代表这些第三方组件授予了额外商业授权。

详见上游 `README.md` 的「授权说明」章节与 `THIRD_PARTY_NOTICES.md`。

## 本仓库相对上游的改动

仅为在本机构建与运行所做的兼容性修复，**不涉及功能改动**：

1. `src-tauri/src/window_effects.rs` —— 上游使用的 `window-shadows-v2` 0.1.1 在 Tauri 2.10 下对窗口句柄未就绪的情况无防护（直接 `unwrap()`），导致启动崩溃。改为通过 `run_on_main_thread` 延后执行，并复用项目已有的守卫式 DWM 调用。
2. `src-tauri/Cargo.toml` / `Cargo.lock` —— 移除上面那个有问题的 `window-shadows-v2` 依赖。
3. `vite.config.ts` / `vite.config.js` —— `server.host` 由 `false` 改为 `"127.0.0.1"`，避免 Windows 下 Vite 只监听 IPv6 `::1` 而 Tauri WebView2 连不上 `localhost:1420`。
4. `src-tauri/tauri.conf.json` —— 新增 `bundle.useLocalToolsDir: true`，让 NSIS 打包工具缓存到 `target/.tauri/`，绕开受限的 `%LOCALAPPDATA%`。

## 构建产物

安装包 `EAiCoding_0.1.10_x64-setup.exe`（259.6 MB）请从本仓库的 **Releases** 页面下载，不随源码提交。

SHA256: `123BFB5C4D88F0CC17E1137BA50A0D895F86C0406F7513C0B2A9AF66543DDD76`
