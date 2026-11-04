四个公开的 val 供你 remix，每个对应本指南的一部分。

**[带评论的博客](https://www.val.town/x/fastcomments/blog-with-comments)**（[在线演示](https://fastcomments-blog.val.run)）是一个 Markdown 博客，每篇文章下都有一个评论线程，索引页上显示批量评论计数。remix 后即可立即使用，只需一个环境变量即可指向你自己的账户。

**[SSO 演示](https://www.val.town/x/fastcomments/sso-demo)**（[在线演示](https://fastcomments-sso.val.run)）使用访客的 Val Town 账户登录，并将该身份传递给组件，因此无需二次登录。

**[Webhook 接收器](https://www.val.town/x/fastcomments/webhook-receiver)**（[在线演示](https://fastcomments-webhooks.val.run)）会验证每次投递的 HMAC 签名，并将事件存储在 SQLite 中。它有一个按钮，可以对测试载荷进行签名并投递给自己，因此你可以在配置真实 webhook 之前看到验证成功的过程。

**[Agent 技能](https://www.val.town/x/fastcomments/skills)**（[在线演示](https://fastcomments-skills.val.run)）是一个 FastComments agent 技能库，涵盖组件、SSO、REST API、审核以及从 Disqus 迁移。remix 后，Val Town 的 agent Townie 会自动从 `skills/` 中获取这些技能，因此你的 agent 无需你将文档粘贴到聊天中就知道如何接入评论。

同样的技能也可以通过 `npx skills add fastcomments/skills` 安装到其他任何地方。