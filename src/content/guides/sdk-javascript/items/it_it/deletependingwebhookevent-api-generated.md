## Parametri

| Name | Type | Richiesto | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di deletePendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_42f7c9b1';
  const id: string = 'pending_webhook_ev_8f3b9a2d';
  const reason?: string = undefined; // esempio di parametro opzionale (non richiesto dalla funzione)
  const result: FlagCommentPublic200Response = await deletePendingWebhookEvent(tenantId, id);
  console.log(result);
})();
[inline-code-end]

---