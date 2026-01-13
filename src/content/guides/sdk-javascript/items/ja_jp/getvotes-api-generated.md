## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |

## レスポンス

戻り値: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotes200Response.ts)

## 例

[inline-code-attrs-start title = 'getVotes の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b3c_prod';
const urlId: string = '/news/2026/typescript-ecosystem-update';
const votes: GetVotes200Response = await getVotes(tenantId, urlId);
// オプションのパラメータが存在する場合（例: includeHidden）、次のように使用するかもしれません:
// const votesWithHidden: GetVotes200Response = await getVotes(tenantId, urlId, { includeHidden: true });
[inline-code-end]

---