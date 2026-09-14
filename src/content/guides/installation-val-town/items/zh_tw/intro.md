[Val Town](https://val.town) 在 Deno 上執行 TypeScript，因此 val 是一個真實的伺服器。這使它非常適合 FastComments：小工具是頁面上的 script 標籤，任何需要機密的功能，如 Secure SSO 或驗證 webhook，都可以在同一個 val 的伺服器端執行。

本指南說明如何將評論小工具加入 HTTP val、在索引頁面顯示評論計數、使用他們已擁有的 Val Town 帳號登入使用者，以及接收評論 webhook。

您不需要帳號即可試用。範例使用 `tenantId: "demo"`，這是一個共享的 sandbox，第二步說明如何切換到您自己的環境。