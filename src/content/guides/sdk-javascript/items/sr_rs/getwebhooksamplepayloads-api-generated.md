Недавни коментари у тачном облику који користе webhook доставе, за израду интеграција (на пример Zapier
пример података). Сваки догађај доставља исти објекат коментара, тако да `event` само мора бити валидан.

## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| event | WebhookEventName | Не |  |
| limit | number | Не |  |

## Одговор

Враћа: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Пример

[inline-code-attrs-start title = 'getWebhookSamplePayloads пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getWebhookSamplePayloads(
  tenantId: string,
  event?: WebhookEventName,
  limit?: number
): Promise<GetWebhookSamplePayloadsResponse>;

const tenantId: string = '5f8d9b3e2a1b4c6d7e8f9a0b';

// Преузми примере payload-ова за одређени тип догађаја
const commentPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  WebhookEventName.COMMENT_SUCCESS_PAGE,
  10
);

// Преузми све примере payload-ова (без опционих параметара)
const allPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(tenantId);

console.log(`Comment payload status: ${commentPayloads.status}`);
console.log(`Fetched ${allPayloads.payloads.length} sample payloads`);
[inline-code-end]