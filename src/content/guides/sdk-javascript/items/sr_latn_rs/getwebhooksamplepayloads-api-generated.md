Skorašnji komentari u tačnom obliku koji koriste isporuke webhook‑ova, za izgradnju integracija (na primer Zapier uzorak podataka). Svaki događaj isporučuje isti objekat komentara, tako da `event` mora biti validan.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| event | WebhookEventName | Ne |  |
| limit | number | Ne |  |

## Odgovor

Vraća: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getWebhookSamplePayloads'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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