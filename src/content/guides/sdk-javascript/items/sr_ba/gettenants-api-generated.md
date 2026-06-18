## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| meta | string | Ne |  |
| skip | number | Ne |  |

## Odgovor

Vraća: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenants200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getTenants'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fcom-tenant-8b4f2a1c";
const meta: string = "include=domains,billing&status=active";
const skip: number = 20;
const response: GetTenants200Response = await getTenants(tenantId, meta, skip);
console.log(response);
[inline-code-end]

---