## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentId | string | כן |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | כן |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnblockSuccess.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-unBlockCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-72';
const commentId: string = 'cmt_5f9b3a2d';
const publicBlockFromCommentParams: PublicBlockFromCommentParams = {};
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso_payload.sig';
const result: UnblockSuccess = await unBlockCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
[inline-code-end]

---