## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| updateTenantUserBody | UpdateTenantUserBody | Da |  |
| updateComments | string | Ne |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer updateTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8f4a2b7c";
const id: string = "user_74d9c1a3";
const updateTenantUserBody: UpdateTenantUserBody = {
  email: "jane.doe@acme-corp.com",
  displayName: "Jane Doe",
  roles: ["moderator"],
  active: true
};
const updateComments: string = "Promoted to moderator for customer support";
const response: APIEmptyResponse = await updateTenantUser(tenantId, id, updateTenantUserBody, updateComments);
[inline-code-end]