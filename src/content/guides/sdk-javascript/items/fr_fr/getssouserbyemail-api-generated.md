## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| email | string | Oui |  |

## Réponse

Renvoie : [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUserByEmailAPIResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getSSOUserByEmail'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchSSOUser() {
  const tenantId: string = "tenant_12345";
  const email: string = "john.doe@example.com";

  const result: GetSSOUserByEmailAPIResponse = await getSSOUserByEmail(tenantId, email);
  const user: APISSOUser | undefined = result?.user;
}
[inline-code-end]

---