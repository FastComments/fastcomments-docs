---
此 SDK 使用 **双入口点** 以确保最佳兼容性并防止运行时错误：

- **`fastcomments-sdk/browser`** - 浏览器安全版本，使用原生 `fetch`
- **`fastcomments-sdk/server`** - 完整的 Node.js 版本，支持 SSO
- **`fastcomments-sdk`** (默认) - 仅类型，可在任何地方安全导入
---