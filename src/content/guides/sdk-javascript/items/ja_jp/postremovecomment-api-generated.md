## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## レスポンス

返却: [`PostRemoveCommentApiResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostRemoveCommentApiResponse.ts)

## 例

[inline-code-attrs-start title = 'postRemoveComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---