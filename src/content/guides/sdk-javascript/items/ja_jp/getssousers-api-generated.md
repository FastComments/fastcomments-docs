## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| skip | number | いいえ |  |

## レスポンス

戻り値: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUsersResponse.ts)

## 例

[inline-code-attrs-start title = 'getSSOUsers の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8f3b2a1c";
const usersWithoutSkip: GetSSOUsersResponse = await getSSOUsers(tenantId);
const skip: number = 50;
const usersWithSkip: GetSSOUsersResponse = await getSSOUsers(tenantId, skip);
[inline-code-end]

---