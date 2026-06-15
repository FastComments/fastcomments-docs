## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Antwort

Gibt zurück: [`GetUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUser200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments_corp';
const id: string = 'user_9f8b7c6d-5e4a-3b2c-1f0e-123456789abc';
const response: GetUser200Response = await getUser(tenantId, id);
const userEmail: string | undefined = response.user?.email;
const displayName: string | undefined = response.user?.displayName
[inline-code-end]

---