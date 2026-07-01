## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| updateTenantBody | UpdateTenantBody | Oui |  |

## Réponse

Retourne : [`UpdateTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateTenantResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple updateTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c8f9e3d2-4b6a-11ee-8c99-0242ac130003";
const id: string = "tenant-config-01";

const updateBody: UpdateTenantBody = {
  domain: "mytenant.fastcomments.io",
  branding: {
    logoUrl: "https://cdn.mytenant.com/assets/logo.png"
  },
  description: "Branding update for Q3"
};

const response: UpdateTenantResponse = await updateTenant(tenantId, id, updateBody);
console.log(response);
[inline-code-end]