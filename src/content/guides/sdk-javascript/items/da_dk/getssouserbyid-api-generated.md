## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Svar

Returnerer: [`GetSSOUserByIdAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUserByIdAPIResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getSSOUserById Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUser() {
  const tenantId: string = "tenant-987654321";
  const userId: string = "sso-user-abc123";
  const result: GetSSOUserByIdAPIResponse = await getSSOUserById(tenantId, userId);
  const ssoUser: APISSOUser = result.user;
}
[inline-code-end]