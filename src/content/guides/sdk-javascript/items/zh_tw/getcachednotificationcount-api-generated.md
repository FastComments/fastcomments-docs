## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

返回: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCachedNotificationCountResponse.ts)

## 範例

[inline-code-attrs-start title = 'getCachedNotificationCount 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const id: string = "user_9876";
  const response: GetCachedNotificationCountResponse = await getCachedNotificationCount(tenantId, id);
})();
[inline-code-end]

---