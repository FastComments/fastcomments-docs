Fire offentlige vals, du kan remixe, som hver især dækker en del af denne guide.

**[Blog med kommentarer](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) er en Markdown-blog med en tråd under hvert opslag og samlede kommentartællinger på indekssiden. Den virker, så snart du remixer den, og én miljøvariabel peger den på din egen konto.

**[SSO-demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) logger besøgende ind med deres Val Town-konto og giver denne identitet videre til widgetten, så der ikke er behov for et andet login.

**[Webhook-modtager](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) verificerer HMAC-signaturen ved hver levering og gemmer events i SQLite. Den har en knap, der signerer en test-payload og leverer den til sig selv, så du kan se verifikationen lykkes, før du konfigurerer en rigtig webhook.

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) er et bibliotek af FastComments agent skills, der dækker widgetten, SSO, REST API'et, moderation og migrering væk fra Disqus. Remix den, og Val Towns agent, Townie, henter automatisk skills fra `skills/`, så din agent ved, hvordan man sætter kommentarer op, uden at du skal indsætte dokumentation i chatten.

De samme skills kan installeres andre steder med `npx skills add fastcomments/skills`.