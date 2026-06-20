## パラメーター

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| sso | string | 任意 |  |

## レスポンス

戻り値: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIModerateGetUserBanPreferencesResponse.ts)

## 例

[inline-code-attrs-start title = 'getUserBanPreference の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const ssoToken: string = "sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
const resultWithoutSso: APIModerateGetUserBanPreferencesResponse = await getUserBanPreference();
const resultWithSso: APIModerateGetUserBanPreferencesResponse = await getUserBanPreference(ssoToken);
[inline-code-end]

---