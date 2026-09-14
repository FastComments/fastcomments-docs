Four public vals you can remix, each covering one piece of this guide.

**[Blog med kommentarer](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) er en Markdown-blog med en tråd under hvert indlæg og samlede kommentarantal på indeks-siden. Den fungerer i det øjeblik, du remix'er den, og én miljøvariabel peger den på din egen konto.

**[SSO-demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) logger besøgende ind med deres Val Town-konto og overlever den identitet til widget'en, så der ikke er noget andet login.

**[Webhook-modtager](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) verificerer HMAC-signaturen på hver levering og gemmer hændelser i SQLite. Den har en knap, der underskriver en testpayload og leverer den til sig selv, så du kan se verifikationen lykkes, før du konfigurerer en rigtig webhook.

**[Agent færdigheder](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) er et bibliotek af FastComments-agentfærdigheder, der dækker widget'en, SSO, REST API'et, moderation og migrering væk fra Disqus. Remix den, og Val Town's agent, Townie, henter færdighederne fra `skills/` automatisk, så din agent ved, hvordan man integrerer kommentarer uden at du skal indsætte dokumentation i chatten.

De samme færdigheder kan installeres hvor som helst andet med `npx skills add fastcomments/skills`.