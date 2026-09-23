## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| userId | string | いいえ |  |
| anonUserId | string | いいえ |  |

## レスポンス

返り値: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesForUserResponse.ts)

## 例

[inline-code-attrs-start title = 'getVotesForUser の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "post_9876";
  const userId: string = "user_abcde";
  const anonUserId: string = "anon_5678";

  const responseWithUser: GetVotesForUserResponse = await getVotesForUser(tenantId, urlId, userId);
  const responseWithAnon: GetVotesForUserResponse = await getVotesForUser(tenantId, urlId, undefined, anonUserId);
}

demo();
[inline-code-end]