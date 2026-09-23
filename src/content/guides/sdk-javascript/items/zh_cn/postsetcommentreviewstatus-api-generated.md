## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| reviewed | boolean | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## 响应

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 示例

[inline-code-attrs-start title = 'postSetCommentReviewStatus 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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