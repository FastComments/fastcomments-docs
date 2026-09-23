## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 否 |  |
| sso | string | 否 |  |

## 回應

Returns: [`PostRemoveCommentApiResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostRemoveCommentApiResponse.ts)

## 範例

[inline-code-attrs-start title = 'postRemoveComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removeCommentExample() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const broadcastId: string = "brd_555";
  const sso: string = "sso_token_abc";

  const response: PostRemoveCommentApiResponse = await postRemoveComment(
    tenantId,
    commentId,
    broadcastId,
    sso
  );
  console.log(response);
}

removeCommentExample();
[inline-code-end]