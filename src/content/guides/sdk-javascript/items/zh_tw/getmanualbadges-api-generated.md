## 參數

| 名稱 | 類型 | 必要 | 描述 |
|------|------|----------|-------------|
| sso | string | 否 |  |

## 回應

回傳: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantManualBadgesResponse.ts)

## 範例

[inline-code-attrs-start title = 'getManualBadges 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI1Njc4OSIsImlhdCI6MTYwOTQyNjQwMH0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
const manualBadgesWithSso: GetTenantManualBadgesResponse = await getManualBadges(ssoToken);
const manualBadgesWithoutSso: GetTenantManualBadgesResponse = await getManualBadges();
[inline-code-end]

---