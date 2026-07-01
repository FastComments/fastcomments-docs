## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| commentId | string | 是 |  |
| setCommentTextParams | SetCommentTextParams | 是 |  |
| broadcastId | string | 否 |  |
| tenantId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`PostSetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostSetCommentTextResponse.ts)

## 示例

[inline-code-attrs-start title = 'postSetCommentText 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const commentId: string = "cmt_9f8b7a6d5e4c3b2a1";
  const setParams: SetCommentTextParams = {
    text: "Edited comment to correct a typo and add clarification."
  };
  const broadcastId: string = "brd_1a2b3c4d5e6f7g8h9";
  const tenantId: string = "tenant_xyz123";
  const sso: string = "sso_user_456def";

  const fullResult: PostSetCommentTextResponse = await postSetCommentText(
    commentId,
    setParams,
    broadcastId,
    tenantId,
    sso
  );

  const minimalResult: PostSetCommentTextResponse = await postSetCommentText(
    commentId,
    setParams
  );

  console.log(fullResult, minimalResult);
})();
[inline-code-end]

---