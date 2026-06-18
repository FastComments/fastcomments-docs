## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של deletePendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_5f8d7a34";
const id: string = "webhook_evt_987654321";
const requestNote: string | undefined = undefined;
const response: FlagCommentPublic200Response = await deletePendingWebhookEvent(tenantId, id);
[inline-code-end]

---