## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| sso | string | No |  |

## Respuesta

Devuelve: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIModerateGetUserBanPreferencesResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getUserBanPreference'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const ssoToken: string = "sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
const resultWithoutSso: APIModerateGetUserBanPreferencesResponse = await getUserBanPreference();
const resultWithSso: APIModerateGetUserBanPreferencesResponse = await getUserBanPreference(ssoToken);
[inline-code-end]

---