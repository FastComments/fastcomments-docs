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
| skip | number | 아니요 |  |

## 응답

반환: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEvents200Response.ts)

## 예제

[inline-code-attrs-start title = 'getPendingWebhookEvents 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_78b2f1";
const commentId: string = "cmt_0042";
const eventType: string = "comment.created";
const domain: string = "blog.example.com";
const attemptCountGT: number = 1;
const skip: number = 0;

const pending: GetPendingWebhookEvents200Response = await getPendingWebhookEvents(
  tenantId,
  commentId,
  undefined, // externalId (외부 ID)
  eventType,
  undefined, // type (유형)
  domain,
  attemptCountGT,
  skip
);
[inline-code-end]

---