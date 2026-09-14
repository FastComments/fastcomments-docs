Four public vals you can remix, each covering one piece of this guide.

**[Blog with comments](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) is a Markdown blog with a thread under every post and bulk comment counts on the index. It works the moment you remix it, and one environment variable points it at your own account.

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) signs the visitor in with their Val Town account and hands that identity to the widget, so there is no second login.

**[Webhook receiver](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) verifies the HMAC signature on every delivery and stores events in SQLite. It has a button that signs a test payload and delivers it to itself, so you can watch verification succeed before configuring a real webhook.

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) is a library of FastComments agent skills covering the widget, SSO, the REST API, moderation, and migrating off Disqus. Remix it and Val Town's agent, Townie, picks the skills up out of `skills/` automatically, so your agent knows how to wire up comments without you pasting documentation into the chat.

The same skills install anywhere else with `npx skills add fastcomments/skills`.
