## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIModerateGetUserBanPreferencesResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUserBanPreference'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const ssoToken: string = "sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
const resultWithoutSso: APIModerateGetUserBanPreferencesResponse = await getUserBanPreference();
const resultWithSso: APIModerateGetUserBanPreferencesResponse = await getUserBanPreference(ssoToken);
[inline-code-end]

---