---
[Val Town](https://val.town) 在 Deno 上运行 TypeScript，因此 val 是一个真实的服务器。这使它非常适合 FastComments：小部件是页面上的 script 标签，任何需要密钥的功能，如 Secure SSO 或验证 webhook，都可以在同一个 val 的服务器端运行。

本指南涵盖了将评论小部件添加到 HTTP val、在索引页面显示评论计数、使用用户已有的 Val Town 账户登录以及接收评论 webhook。

您无需账户即可尝试。示例使用 `tenantId: "demo"`，这是一个共享的沙盒环境，第 2 步将介绍如何切换到您自己的。

---