---
此 SDK 使用 **雙入口點** 以確保最佳相容性並避免執行時錯誤：

- **`fastcomments-sdk/browser`** - 瀏覽器安全的版本，使用原生 `fetch`
- **`fastcomments-sdk/server`** - 完整的 Node.js 版本，支援 SSO
- **`fastcomments-sdk`** (default) - 僅型別，能安全在任何地方匯入
---