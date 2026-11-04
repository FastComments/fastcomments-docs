四個公開的 val 供您 remix，每個涵蓋本指南的一部分。

**[Blog with comments](https://www.val.town/x/fastcomments/blog-with-comments)**（[線上版](https://fastcomments-blog.val.run)）是一個 Markdown 部落格，每篇文章下方都有留言串，並在索引頁顯示大量留言數。Remix 後立即可用，只需一個環境變數即可指向您自己的帳號。

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)**（[線上版](https://fastcomments-sso.val.run)）會使用訪客的 Val Town 帳號登入，並將該身分傳遞給小工具，因此無需第二次登入。

**[Webhook receiver](https://www.val.town/x/fastcomments/webhook-receiver)**（[線上版](https://fastcomments-webhooks.val.run)）會驗證每次傳送的 HMAC 簽章，並將事件儲存在 SQLite 中。它提供一個按鈕，可簽署測試酬載並傳送給自己，讓您在設定真實 webhook 之前先看到驗證成功的過程。

**[Agent skills](https://www.val.town/x/fastcomments/skills)**（[線上版](https://fastcomments-skills.val.run)）是一個 FastComments agent skills 函式庫，涵蓋小工具、SSO、REST API、審核，以及從 Disqus 遷移。Remix 之後，Val Town 的 agent Townie 會自動從 `skills/` 目錄取得這些 skills，因此您的 agent 無需您將文件貼到聊天中，就知道如何設定留言功能。

同樣的 skills 也可以在其他地方使用 `npx skills add fastcomments/skills` 安裝。