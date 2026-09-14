Four public vals you can remix, each covering one piece of this guide.

**[Blog met reacties](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) is een Markdown-blog met een thread onder elk bericht en bulkreactietellingen op de index. Het werkt meteen nadat je het remixt, en één omgevingsvariabele wijst het naar je eigen account.

**[SSO-demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) meldt de bezoeker aan met hun Val Town-account en geeft die identiteit door aan de widget, zodat er geen tweede login nodig is.

**[Webhook-ontvanger](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) verifieert de HMAC-handtekening bij elke levering en slaat gebeurtenissen op in SQLite. Het heeft een knop die een testpayload ondertekent en naar zichzelf verzendt, zodat je de verificatie kunt zien slagen voordat je een echte webhook configureert.

**[Agent-vaardigheden](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) is een bibliotheek van FastComments-agentvaardigheden die de widget, SSO, de REST API, moderatie en het migreren van Disqus behandelen. Remix het en de agent van Val Town, Townie, haalt de vaardigheden automatisch uit `skills/`, zodat je agent weet hoe commentaren moeten worden opgezet zonder dat je documentatie in de chat plakt.

Dezelfde vaardigheden kun je overal anders installeren met `npx skills add fastcomments/skills`.