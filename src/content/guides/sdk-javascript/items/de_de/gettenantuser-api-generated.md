## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Antwort

Gibt zurück: [`GetTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUser200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_fc5a9b2c';
const userId: string = 'user_0a12b3';
const result: GetTenantUser200Response = await getTenantUser(tenantId, userId);
const user: User | undefined = (result as any).user; // accessing payload
const userEmail: string | undefined = user?.email;
console.log('Fetched user email:', userEmail);
[inline-code-end]

---