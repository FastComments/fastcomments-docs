## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'unBlockCommentPublic 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-42-production";
const commentId: string = "comment_7f3b2a9d";
const publicBlockFromCommentParams: PublicBlockFromCommentParams = {
  reason: "flag reviewed and determined not to violate policy",
  restoredBy: "moderator_jane",
  restoredAt: new Date().toISOString()
};
const sso: string = "sso_token_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
const result: UnBlockCommentPublic200Response = await unBlockCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
[inline-code-end]

---