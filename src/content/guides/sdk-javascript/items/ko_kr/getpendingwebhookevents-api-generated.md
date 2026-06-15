---
## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 아니오 |  |
| externalId | string | 아니오 |  |
| eventType | string | 아니오 |  |
| type | string | 아니오 |  |
| domain | string | 아니오 |  |
| attemptCountGT | number | 아니오 |  |
| skip | number | 아니오 |  |

## 응답

반환: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEvents200Response.ts)

## 예제

[inline-code-attrs-start title = 'getPendingWebhookEvents 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b2a';
const commentId: string = 'cmt_8a7d1';
const eventType: string = 'comment.created';
const domain: string = 'reviews.myshop.com';
const attemptCountGT: number = 1;
const skip: number = 0;

const result: GetPendingWebhookEvents200Response = await getPendingWebhookEvents(
  tenantId,
  commentId,
  undefined,
  eventType,
  undefined,
  domain,
  attemptCountGT,
  skip
);
[inline-code-end]

---