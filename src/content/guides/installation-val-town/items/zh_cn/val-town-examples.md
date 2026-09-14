Four public vals you can remix, each covering one piece of this guide.

**[Blog with comments](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) 是一个 Markdown 博客，每篇文章下都有一个线程，并在索引页显示批量评论计数。它在你 remix 的瞬间即可工作，只需一个环境变量即可指向你的账户。

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) 使用访客的 Val Town 账户登录，并将该身份传递给小部件，因此无需二次登录。

**[Webhook receiver](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) 对每次交付的 HMAC 签名进行验证，并将事件存储在 SQLite 中。它有一个按钮可以对测试负载进行签名并发送给自身，这样你可以在配置真实 webhook 之前看到验证成功。

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) 是一个 FastComments 代理技能库，涵盖小部件、SSO、REST API、审核以及从 Disqus 迁移。Remix 它后，Val Town 的代理 Townie 会自动从 `skills/` 中加载这些技能，使你的代理能够在不需要你将文档粘贴到聊天中的情况下，了解如何接入评论。

The same skills install anywhere else with `npx skills add fastcomments/skills`.