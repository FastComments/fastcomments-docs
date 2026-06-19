## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCachedNotificationCountResponse.ts)

## 例

[inline-code-attrs-start title = 'getCachedNotificationCount の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f3a2b4c";
const id: string = "user_9812b";
const result: GetCachedNotificationCountResponse = await getCachedNotificationCount(tenantId, id);
[inline-code-end]

---