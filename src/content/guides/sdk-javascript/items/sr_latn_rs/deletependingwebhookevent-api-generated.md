## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Primer

[inline-code-attrs-start title = 'deletePendingWebhookEvent Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f3b2a";
const webhookEventId: string = "wh_evt_9a8c7d1234";
const dryRun: boolean | undefined = undefined; // primer opcionog flag-a (nije obavezno za ovaj poziv)
const result: FlagCommentPublic200Response = await deletePendingWebhookEvent(tenantId, webhookEventId);
[inline-code-end]

---