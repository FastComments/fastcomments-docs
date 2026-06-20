---
## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIModerateGetUserBanPreferencesResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getUserBanPreference'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const ssoToken: string = "sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
const resultWithoutSso: APIModerateGetUserBanPreferencesResponse = await getUserBanPreference();
const resultWithSso: APIModerateGetUserBanPreferencesResponse = await getUserBanPreference(ssoToken);
[inline-code-end]

---