Недави коментари у тачној структури коју користе испоруке веб‑хукова, за изградњу интеграција (на пример Zapier пример података). Сваки догађај испоручује исти објекат коментара, тако да `event` мора бити валидан.

## Parameters

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Да |  |
| event | WebhookEventName | Не |  |
| limit | number | Не |  |

## Response

Враћа: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Пример

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