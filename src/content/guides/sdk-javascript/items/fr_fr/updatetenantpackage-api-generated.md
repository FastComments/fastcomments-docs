## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Oui |  |

## Réponse

Retourne : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de updateTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";
const packageId: string = "pkg_3a2b1c";

const updateBody: UpdateTenantPackageBody = {
  // les champs optionnels peuvent être omis ou inclus selon les besoins
  // newPackageName?: string;
  // renewalDate?: string;
};

const result: APIEmptyResponse = await updateTenantPackage(tenantId, packageId, updateBody);
[inline-code-end]

---