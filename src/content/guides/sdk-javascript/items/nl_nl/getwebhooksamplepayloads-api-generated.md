Recente opmerkingen in precies de vorm die webhook‑leveringen gebruiken, voor het bouwen van integraties (bijvoorbeeld Zapier‑voorbeeldgegevens). Elk evenement levert hetzelfde opmerkingobject, dus `event` hoeft alleen geldig te zijn.

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| event | WebhookEventName | Nee |  |
| limit | number | Nee |  |

## Response

Retourneert: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getWebhookSamplePayloads Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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