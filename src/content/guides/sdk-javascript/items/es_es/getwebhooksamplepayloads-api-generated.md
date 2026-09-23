Comentarios recientes en exactamente la forma que usan las entregas de webhook, para crear integraciones (por ejemplo datos de muestra de Zapier). Cada evento entrega el mismo objeto de comentario, por lo que `event` solo tiene que ser válido.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| event | WebhookEventName | No |  |
| limit | number | No |  |

## Response

Devuelve: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Example

[inline-code-attrs-start title = 'Ejemplo getWebhookSamplePayloads'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";

const responseAll: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  "comment.created",
  10
);

const responseWithEvent: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  "comment.created"
);

const responseBasic: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId
);
[inline-code-end]