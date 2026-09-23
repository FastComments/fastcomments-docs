Commenti recenti esattamente nella forma che le consegne webhook utilizzano, per costruire integrazioni (ad esempio dati di esempio di Zapier). Ogni evento consegna lo stesso oggetto commento, quindi `event` deve solo essere valido.

## Parameters

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|---------------|-------------|
| tenantId | string | Sì |  |
| event | WebhookEventName | No |  |
| limit | number | No |  |

## Response

Restituisce: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getWebhookSamplePayloads'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---