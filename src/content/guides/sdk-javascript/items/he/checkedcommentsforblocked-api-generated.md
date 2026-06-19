## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentIds | string | כן |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CheckBlockedCommentsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-checkedCommentsForBlocked'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const commentIds: string = 'cmt_1001,cmt_1002';
const resultWithoutSso: CheckBlockedCommentsResponse = await checkedCommentsForBlocked(tenantId, commentIds);

const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.dummy.payload';
const resultWithSso: CheckBlockedCommentsResponse = await checkedCommentsForBlocked(tenantId, commentIds, sso);
[inline-code-end]

---