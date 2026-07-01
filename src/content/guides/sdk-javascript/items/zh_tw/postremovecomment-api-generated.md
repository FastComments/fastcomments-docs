## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| broadcastId | string | 否 |  |
| tenantId | string | 否 |  |
| sso | string | 否 |  |

## 回應

返回：[`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostRemoveCommentResponse.ts)

## 範例

[inline-code-attrs-start title = 'postRemoveComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removeCommentExample(): Promise<void> {
  const commentId: string = "cmt_7f9a3e2b";

  const result1: PostRemoveCommentResponse = await postRemoveComment(commentId);

  const broadcastId: string = "brd_12ab34cd";
  const tenantId: string = "tenant_5678efgh";
  const sso: string = "sso_XYZ12345";

  const result2: PostRemoveCommentResponse = await postRemoveComment(commentId, broadcastId, tenantId, sso);

  console.log(result1, result2);
}
[inline-code-end]