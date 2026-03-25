## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`GetUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUser200Response.ts)

## 例

[inline-code-attrs-start title = 'getUser の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7b3f42';
const id: string = 'user_9c4d2a';
const userResponse: GetUser200Response = await getUser(tenantId, id);
console.log(userResponse);
[inline-code-end]

---