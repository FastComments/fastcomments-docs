---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Antwort

Rückgabe: [`GetSSOUserByIdAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUserByIdAPIResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getSSOUserById Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUser() {
  const tenantId: string = "tenant-987654321";
  const userId: string = "sso-user-abc123";
  const result: GetSSOUserByIdAPIResponse = await getSSOUserById(tenantId, userId);
  const ssoUser: APISSOUser = result.user;
}
[inline-code-end]

---