## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer deletePendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_42f7c9b1';
  const id: string = 'pending_webhook_ev_8f3b9a2d';
  const reason?: string = undefined; // primer opcionog parametra (nije obavezno za funkciju)
  const result: FlagCommentPublic200Response = await deletePendingWebhookEvent(tenantId, id);
  console.log(result);
})();
[inline-code-end]

---