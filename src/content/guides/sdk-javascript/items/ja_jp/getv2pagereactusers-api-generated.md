## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReactUsersResponse.ts)

## 例

[inline-code-attrs-start title = 'getV2PageReactUsers の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fc_tenant_7b4c9d1";
const rawUrlId: string | undefined = undefined; // ルートパラメータから来る可能性があります
const urlId: string = rawUrlId ?? "page-home-9a3f2b";
const id: string = "user_823b5c";

const response: GetV2PageReactUsersResponse = await getV2PageReactUsers(tenantId, urlId, id);
[inline-code-end]

---