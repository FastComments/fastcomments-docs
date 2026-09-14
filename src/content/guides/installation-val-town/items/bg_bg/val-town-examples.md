Четири публични vals, които можете да ремиксирате, всяка от тях обхваща една част от това ръководство.

**[Blog with comments](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) е Markdown блог с нишка под всяка публикация и общи броячи на коментари в индекса. Работи веднага след като го ремиксирате, а една променлива на средата го насочва към вашия собствен акаунт.

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) вписва посетителя с техния Val Town акаунт и предава тази идентичност на уиджета, така че няма второ влизане.

**[Webhook receiver](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) проверява HMAC подписа при всяка доставка и съхранява събития в SQLite. Има бутон, който подписва тестово натоварване и го изпраща към себе си, така че можете да видите проверката да успее преди да конфигурирате истински webhook.

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) е библиотека от FastComments агентски умения, обхващащи уиджета, SSO, REST API, модериране и миграция от Disqus. Ремиксирайте я и агентът на Val Town, Townie, автоматично взема уменията от `skills/`, така че вашият агент знае как да настрои коментарите без да копирате документацията в чата.

Същите умения се инсталират навсякъде другаде с `npx skills add fastcomments/skills`.