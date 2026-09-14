### Ressourceforbrug

Det skal bemærkes, at hentning af data fra API'et tælles som forbrug på din konto.

Hver ressource vil angive, hvad dette forbrug er, i sin egen sektion.

Nogle ressourcer koster mere at betjene end andre. Hvert endpoint har en fast pris i kreditter pr. API‑opkald. For nogle endpoints varierer antallet af kreditter afhængigt af indstillingerne og svarstørrelserne.

API‑forbruget kan kontrolleres på siden [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) og opdateres hver få minutter.

#### Bemærk!

Vi foreslår, at du læser Pages-dokumentationen først for at begrænse forvirring, når du skal bestemme, hvilke værdier du skal sende for `urlId` i Comment‑API'et.

### Webhooks

Webhook-abonnementer har deres egen vejledning. `POST`, `GET` og `DELETE /api/v1/webhooks` samt `GET /api/v1/webhooks/sample-payloads` er dokumenteret under [Managing Subscriptions via API](/guide-webhooks.html#webhooks-api-subscriptions), og hændelses‑payloads under [Webhook Structures](/guide-webhooks.html#webhooks-structures).

---