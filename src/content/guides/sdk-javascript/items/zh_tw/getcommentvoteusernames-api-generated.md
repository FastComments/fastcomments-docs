---
## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| dir | number | 是 |  |
| sso | string | 否 |  |

## 回應

回傳：[`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentVoteUserNames200Response.ts)

## 範例

[inline-code-attrs-start title = 'getCommentVoteUserNames 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_67890";
const commentId: string = "comment_abc123";
const dir: number = 1;
const ssoToken: string = "sso-eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";

const responseWithoutSSO: GetCommentVoteUserNames200Response = await getCommentVoteUserNames(tenantId, commentId, dir);
const responseWithSSO: GetCommentVoteUserNames200Response = await getCommentVoteUserNames(tenantId, commentId, dir, ssoToken);
[inline-code-end]

---