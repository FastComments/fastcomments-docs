Nedavni komentari u tačno onom obliku koji koriste webhook isporuke, za izgradnju integracija (na primer Zapier
primer podataka). Svaki događaj isporučuje isti objekat komentara, tako da `event` samo mora biti validan.

## Parametri

| Naziv | Tip | Obavezan | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| event | WebhookEventName | Ne |  |
| limit | number | Ne |  |

## Odgovor

Vraća: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Primer

[inline-code-attrs-start title = 'getWebhookSamplePayloads Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getWebhookSamplePayloads(
  tenantId: string,
  event?: WebhookEventName,
  limit?: number
): Promise<GetWebhookSamplePayloadsResponse>;

const tenantId: string = '5f8d9b3e2a1b4c6d7e8f9a0b';

// Get sample payloads for a specific event type
const commentPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  WebhookEventName.COMMENT_SUCCESS_PAGE,
  10
);

// Get all sample payloads (no optional parameters)
const allPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(tenantId);

console.log(`Comment payload status: ${commentPayloads.status}`);
console.log(`Fetched ${allPayloads.payloads.length} sample payloads`);
[inline-code-end]