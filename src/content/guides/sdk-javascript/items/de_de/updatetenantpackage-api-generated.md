---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Yes |  |

## Antwort

Rückgabe: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'updateTenantPackage Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";
const packageId: string = "pkg_3a2b1c";

const updateBody: UpdateTenantPackageBody = {
  // optionale Felder können weggelassen oder bei Bedarf hinzugefügt werden
  // newPackageName?: string;
  // renewalDate?: string;
};

const result: APIEmptyResponse = await updateTenantPackage(tenantId, packageId, updateBody);
[inline-code-end]

---