Four public vals you can remix, each covering one piece of this guide.

**[Blog with comments](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) 是一個 Markdown 部落格，每篇文章下都有討論串，索引頁顯示大量評論計數。它在您 remix 的瞬間即可運作，且只需一個環境變數即可指向您自己的帳號。

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) 使用訪客的 Val Town 帳號登入，並將該身份傳遞給小部件，因而不需要第二次登入。

**[Webhook receiver](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) 在每次傳遞時驗證 HMAC 簽名，並將事件儲存於 SQLite。它有一個按鈕可簽署測試 payload 並自行傳送，讓您在設定真實 webhook 前看到驗證成功。

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) 是一個 FastComments 代理技能庫，涵蓋小部件、SSO、REST API、審核以及從 Disqus 遷移。Remix 它後，Val Town 的代理 Townie 會自動從 `skills/` 取得這些技能，讓您的代理知道如何設定評論，而不必把文件貼到聊天中。

The same skills install anywhere else with `npx skills add fastcomments/skills`.