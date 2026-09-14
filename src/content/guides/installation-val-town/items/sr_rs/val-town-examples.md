Four public vals you can remix, each covering one piece of this guide.

**[Blog with comments](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) је Markdown блог са темом испод сваког поста и збирним бројем коментара на индексу. Ради одмах када га ремиксате, а једна променљива окружења указује на ваш налог.

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) пријављује посетиоца помоћу њиховог Val Town налога и прослеђује тај идентитет виџету, тако да не постоји друга пријава.

**[Webhook receiver](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) верификује HMAC потпис на свакој испоруци и чува догађаје у SQLite. Има дугме које потписује тестни податак и испоручује га самом себи, тако да можете видети да верификација успе пре него што подесите прави вебхоок.

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) је библиотека FastComments агентских вештина које обухватају виџет, SSO, REST API, модерацију и миграцију са Disqus‑а. Ремиксујте је и агент Val Town‑а, Townie, аутоматски преузима вештине из `skills/`, тако да ваш агент зна како да постави коментаре без да ви лепите документацију у ћаскање.

The same skills install anywhere else with `npx skills add fastcomments/skills`.