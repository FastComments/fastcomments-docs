## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Odgovor

Vrne: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIModerateGetUserBanPreferencesResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getUserBanPreference'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetUserBanPreference() {
  const tenantId: string = "tenant_987654321";
  const ssoToken: string = "sso_user_abc123xyz";

  const responseWithSso: APIModerateGetUserBanPreferencesResponse = await getUserBanPreference(tenantId, ssoToken);
  const responseWithoutSso: APIModerateGetUserBanPreferencesResponse = await getUserBanPreference(tenantId);
}
[inline-code-end]