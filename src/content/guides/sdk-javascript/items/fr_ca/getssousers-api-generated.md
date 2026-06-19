## Paramètres

| Name | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| skip | number | Non |  |

## Réponse

Renvoie: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUsersResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getSSOUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8f3b2a1c";
const usersWithoutSkip: GetSSOUsersResponse = await getSSOUsers(tenantId);
const skip: number = 50;
const usersWithSkip: GetSSOUsersResponse = await getSSOUsers(tenantId, skip);
[inline-code-end]

---