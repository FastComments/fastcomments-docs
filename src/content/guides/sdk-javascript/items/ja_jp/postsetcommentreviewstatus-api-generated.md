## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| reviewed | boolean | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## レスポンス

返却: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 例

[inline-code-attrs-start title = 'postSetCommentReviewStatus 例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";
const reviewed: boolean = true;
const broadcastId: string = "brd_001";
const sso: string = "sso_token_abc";

const result: APIEmptyResponse = await postSetCommentReviewStatus(
  tenantId,
  commentId,
  reviewed,
  broadcastId,
  sso
);
[inline-code-end]