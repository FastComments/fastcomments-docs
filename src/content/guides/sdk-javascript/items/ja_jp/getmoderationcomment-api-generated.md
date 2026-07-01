## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| commentId | string | はい |  |
| includeEmail | boolean | いいえ |  |
| includeIP | boolean | いいえ |  |
| tenantId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`GetModerationCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentResponse.ts)

## 例

[inline-code-attrs-start title = 'getModerationComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchCommentDetails() {
  // 完全なパラメータセット
  const commentId: string = "cmt_12345abc";
  const includeEmail: boolean = true;
  const includeIP: boolean = false;
  const tenantId: string = "tenant_9876";
  const sso: string = "sso_token_xyz";

  const fullResult: GetModerationCommentResponse = await getModerationComment(
    commentId,
    includeEmail,
    includeIP,
    tenantId,
    sso
  );

  // 必須引数のみを使用した最小呼び出し
  const minimalResult: GetModerationCommentResponse = await getModerationComment("cmt_67890def");

  // 必要に応じて結果を使用してください...
}
[inline-code-end]

---