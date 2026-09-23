## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Oui |  |

## Réponse

Retourne : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemple

[inline-code-attrs-start title = 'replaceTenantPackage Exemple'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function replacePackageDemo(): Promise<void> {
  const tenantId: string = "acme-corp-001";
  const packageId: string = "basic-plan-2023";
  const replaceBody: ReplaceTenantPackageBody = {
    newPackageId: "enterprise-plan-2024"
    // les champs optionnels peuvent être ajoutés ici si nécessaire
  };
  const response: APIEmptyResponse = await replaceTenantPackage(tenantId, packageId, replaceBody);
  console.log(response);
}
[inline-code-end]

---