---
## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBannedUsersCountResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getCounts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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