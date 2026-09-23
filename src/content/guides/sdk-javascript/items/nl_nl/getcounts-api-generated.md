---
## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBannedUsersCountResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getCounts Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run() {
  const tenantId: string = "tenant_9876";
  const ssoToken: string = "sso_user_42";

  const resultWithOnlyTenant: GetBannedUsersCountResponse = await getCounts(tenantId);
  const resultWithBoth: GetBannedUsersCountResponse = await getCounts(tenantId, ssoToken);
}

run();
[inline-code-end]

---