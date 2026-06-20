## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| sso | string | 否 |  |

## 回應

回傳: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIModerateGetUserBanPreferencesResponse.ts)

## 範例

[inline-code-attrs-start title = 'getUserBanPreference 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const ssoToken: string = "sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
const resultWithoutSso: APIModerateGetUserBanPreferencesResponse = await getUserBanPreference();
const resultWithSso: APIModerateGetUserBanPreferencesResponse = await getUserBanPreference(ssoToken);
[inline-code-end]

---