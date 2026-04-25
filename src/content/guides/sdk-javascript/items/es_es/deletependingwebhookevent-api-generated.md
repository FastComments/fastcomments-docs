## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |

## Respuesta

Devuelve: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deletePendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f3b2a";
const webhookEventId: string = "wh_evt_9a8c7d1234";
const dryRun: boolean | undefined = undefined; // ejemplo de indicador opcional (no obligatorio para esta llamada)
const result: FlagCommentPublic200Response = await deletePendingWebhookEvent(tenantId, webhookEventId);
[inline-code-end]

---