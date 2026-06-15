## פרמטרים

| Name | Type | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentId | string | לא |  |
| externalId | string | לא |  |
| eventType | string | לא |  |
| type | string | לא |  |
| domain | string | לא |  |
| attemptCountGT | number | לא |  |

## תגובה

מחזיר: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCount200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getPendingWebhookEventCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b3b';
const commentId: string = 'cmt_1a2b3c';
const eventType: string = 'comment.created';
const domain: string = 'news-site.com';
const attemptCountGT: number = 2;

const result: GetPendingWebhookEventCount200Response = await getPendingWebhookEventCount(
  tenantId,
  commentId,
  undefined, // externalId הושמט
  eventType,
  undefined, // type הושמט
  domain,
  attemptCountGT
);
[inline-code-end]

---