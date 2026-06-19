## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | number | 否 |  |

## 回應

回傳: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUsersResponse.ts)

## 範例

[inline-code-attrs-start title = 'getSSOUsers 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8f3b2a1c";
const usersWithoutSkip: GetSSOUsersResponse = await getSSOUsers(tenantId);
const skip: number = 50;
const usersWithSkip: GetSSOUsersResponse = await getSSOUsers(tenantId, skip);
[inline-code-end]

---