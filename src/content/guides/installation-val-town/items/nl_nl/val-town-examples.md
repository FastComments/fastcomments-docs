Vier openbare vals die je kunt remixen, die elk één onderdeel van deze gids behandelen.

**[Blog met reacties](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) is een Markdown-blog met een thread onder elke post en bulkreactietellingen op de index. Het werkt zodra je het remixt, en één omgevingsvariabele wijst het naar je eigen account.

**[SSO-demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) logt de bezoeker in met hun Val Town-account en geeft die identiteit door aan de widget, dus er is geen tweede login nodig.

**[Webhook-ontvanger](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) verifieert de HMAC-handtekening bij elke levering en slaat events op in SQLite. Het heeft een knop die een testpayload ondertekent en aan zichzelf levert, zodat je kunt zien dat verificatie slaagt voordat je een echte webhook configureert.

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) is een bibliotheek met FastComments agent skills die de widget, SSO, de REST API, moderatie en migreren van Disqus beslaan. Remix het en Val Town's agent, Townie, pakt de skills automatisch op uit `skills/`, zodat je agent weet hoe hij reacties moet opzetten zonder dat je documentatie in de chat plakt.

Dezelfde skills installeren overal elders met `npx skills add fastcomments/skills`.