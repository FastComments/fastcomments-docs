Webhook teslimatlarının tam olarak kullandığı biçimde son yorumlar; entegrasyonlar oluşturmak için (örneğin Zapier
örnek verisi). Her olay aynı yorum nesnesini teslim eder, bu nedenle `event` yalnızca geçerli olmak zorundadır.

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| event | WebhookEventName | Hayır |  |
| limit | number | Hayır |  |

## Yanıt

Döndürür: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getWebhookSamplePayloads Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getWebhookSamplePayloads(
  tenantId: string,
  event?: WebhookEventName,
  limit?: number
): Promise<GetWebhookSamplePayloadsResponse>;

const tenantId: string = '5f8d9b3e2a1b4c6d7e8f9a0b';

// Belirli bir olay türü için örnek payload'ları al
const commentPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  WebhookEventName.COMMENT_SUCCESS_PAGE,
  10
);

// Tüm örnek payload'ları al (isteğe bağlı parametre olmadan)
const allPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(tenantId);

console.log(`Comment payload status: ${commentPayloads.status}`);
console.log(`Fetched ${allPayloads.payloads.length} sample payloads`);
[inline-code-end]