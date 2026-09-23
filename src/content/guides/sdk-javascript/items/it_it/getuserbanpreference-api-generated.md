## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| sso | string | No |  |

## Risposta

Restituisce: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIModerateGetUserBanPreferencesResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getUserBanPreference'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetUserBanPreference() {
  const tenantId: string = "tenant_987654321";
  const ssoToken: string = "sso_user_abc123xyz";

  const responseWithSso: APIModerateGetUserBanPreferencesResponse = await getUserBanPreference(tenantId, ssoToken);
  const responseWithoutSso: APIModerateGetUserBanPreferencesResponse = await getUserBanPreference(tenantId);
}
[inline-code-end]