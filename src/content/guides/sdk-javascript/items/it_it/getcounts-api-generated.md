## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Risposta

Restituisce: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBannedUsersCountResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getCounts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run() {
  const tenantId: string = "tenant_9876";
  const ssoToken: string = "sso_user_42";

  const resultWithOnlyTenant: GetBannedUsersCountResponse = await getCounts(tenantId);
  const resultWithBoth: GetBannedUsersCountResponse = await getCounts(tenantId, ssoToken);
}

run();
[inline-code-end]