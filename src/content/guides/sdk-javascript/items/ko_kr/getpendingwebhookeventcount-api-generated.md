## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 아니요 |  |
| externalId | string | 아니요 |  |
| eventType | string | 아니요 |  |
| type | string | 아니요 |  |
| domain | string | 아니요 |  |
| attemptCountGT | number | 아니요 |  |

## 응답

반환: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCount200Response.ts)

## 예제

[inline-code-attrs-start title = 'getPendingWebhookEventCount 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b3b';
const commentId: string = 'cmt_1a2b3c';
const eventType: string = 'comment.created';
const domain: string = 'news-site.com';
const attemptCountGT: number = 2;

const result: GetPendingWebhookEventCount200Response = await getPendingWebhookEventCount(
  tenantId,
  commentId,
  undefined, // externalId 생략
  eventType,
  undefined, // type 생략
  domain,
  attemptCountGT
);
[inline-code-end]

---