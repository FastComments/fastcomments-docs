## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Respuesta

Devuelve: [`DeletePendingWebhookEventResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeletePendingWebhookEventResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo deletePendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDemo() {
  const tenantId: string = 'c3f5e9d2-1a2b-4c3d-9e7f-1234567890ab';
  const eventId: string = 'event_987654321';
  const response: DeletePendingWebhookEventResponse = await deletePendingWebhookEvent(tenantId, eventId);
  console.log(response);
}
runDemo();
[inline-code-end]