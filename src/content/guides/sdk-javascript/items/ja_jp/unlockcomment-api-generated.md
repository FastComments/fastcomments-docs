---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| broadcastId | string | はい |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`UnLockCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnLockCommentResponse.ts)

## 例

[inline-code-attrs-start title = 'unLockComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_9876";
const broadcastId: string = "brd_001";
const ssoToken: string | undefined = "sso_token_abc";

async function run() {
  const unlocked: UnLockCommentResponse = await unLockComment(tenantId, commentId, broadcastId, ssoToken);
  console.log(unlocked);

  // オプションの sso パラメータなしで呼び出す
  const unlockedWithoutSso: UnLockCommentResponse = await unLockComment(tenantId, commentId, broadcastId);
  console.log(unlockedWithoutSso);
}

run();
[inline-code-end]

---