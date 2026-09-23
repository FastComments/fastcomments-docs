## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentId | string | כן |  |
| broadcastId | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'postRestoreDeletedComment דוגמה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";

const resultRequired: APIEmptyResponse = await postRestoreDeletedComment(tenantId, commentId);

const broadcastId: string = "brd_001";
const sso: string = "sso_token_abc123";

const resultAll: APIEmptyResponse = await postRestoreDeletedComment(
  tenantId,
  commentId,
  broadcastId,
  sso
);
[inline-code-end]