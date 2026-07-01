## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| reviewed | boolean | No |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## 応答

返り値: [`PostSetCommentReviewStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostSetCommentReviewStatusResponse.ts)

## 例

[inline-code-attrs-start title = 'postSetCommentReviewStatus の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function updateCommentReviewStatus(): Promise<void> {
  const commentId: string = "cmt_9f8a7b6c5d4e3f2a1b0c";
  const reviewed: boolean = true;
  const broadcastId: string = "broadcast_2024Q1";
  const tenantId: string = "tenant_1001";
  const sso: string = "alice@example.com";

  const response: PostSetCommentReviewStatusResponse = await postSetCommentReviewStatus(
    commentId,
    reviewed,
    broadcastId,
    tenantId,
    sso
  );

  console.log(response);
}
[inline-code-end]

---