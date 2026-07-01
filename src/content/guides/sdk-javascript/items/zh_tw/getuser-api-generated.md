## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

返回：[`GetUserResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserResponse1.ts)

## 範例

[inline-code-attrs-start title = 'getUser 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const userId: string = "user_98765";
  const result: GetUserResponse1 = await getUser(tenantId, userId);
})();
[inline-code-end]

---