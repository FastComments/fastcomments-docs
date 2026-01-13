## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vraća: [`GetTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUser200Response.ts)

## Primjer

[inline-code-attrs-start title = 'getTenantUser Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f7d4b2a-1c3e";
const id: string = "user_6a12b3c4d5";
const includeProfile: boolean | undefined = true; // primjer neobaveznog parametra
const response: GetTenantUser200Response = await getTenantUser(tenantId, id);
console.log("Tenant user fetched", response);
[inline-code-end]

---