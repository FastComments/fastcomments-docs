## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| contextUserId | string | いいえ |  |
| isLive | boolean | いいえ |  |

## レスポンス

戻り値: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteComment200Response.ts)

## 例

[inline-code-attrs-start title = 'deleteComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84a9f2';
const id: string = 'comment_5f3b21';
const contextUserId: string | undefined = 'user_1122';
const isLive: boolean | undefined = true;

async function run(): Promise<void> {
  const result: DeleteComment200Response = await deleteComment(tenantId, id, contextUserId, isLive);
  console.log(result);
}

run();
[inline-code-end]

---