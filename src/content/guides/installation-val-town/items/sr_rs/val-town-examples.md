Четири јавна vals-а које можете ремиксовати, сваки покрива по један део овог водича.

**[Блог са коментарима](https://www.val.town/x/fastcomments/blog-with-comments)** ([уживо](https://fastcomments-blog.val.run)) је Markdown блог са нитима коментара испод сваког поста и укупним бројевима коментара на индексу. Ради чим га ремиксујете, а једна променљива окружења усмерава га на ваш налог.

**[SSO демо](https://www.val.town/x/fastcomments/sso-demo)** ([уживо](https://fastcomments-sso.val.run)) пријављује посетиоца преко његовог Val Town налога и предаје тај идентитет виџету, тако да нема друге пријаве.

**[Пријемник webhook-ова](https://www.val.town/x/fastcomments/webhook-receiver)** ([уживо](https://fastcomments-webhooks.val.run)) верификује HMAC потпис на свакој достави и чува догађаје у SQLite. Има дугме које потписује тестни payload и доставља га самом себи, тако да можете видети успешну верификацију пре него што конфигуришете стварни webhook.

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([уживо](https://fastcomments-skills.val.run)) је библиотека FastComments agent skills-ова који покривају виџет, SSO, REST API, модерацију и миграцију са Disqus-а. Ремиксујте га и Val Town-ов агент, Townie, аутоматски преузима skills-ове из `skills/`, тако да ваш агент зна како да повеже коментаре без тога да лепите документацију у ћаскање.

Исти skills-ови се инсталирају било где другде са `npx skills add fastcomments/skills`.