## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Yes |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Primer

[inline-code-attrs-start title = 'updateTenantPackage Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";
const packageId: string = "pkg_3a2b1c";

const updateBody: UpdateTenantPackageBody = {
  // opcionalna polja mogu biti izostavljena ili uključena po potrebi
  // newPackageName?: string;
  // renewalDate?: string;
};

const result: APIEmptyResponse = await updateTenantPackage(tenantId, packageId, updateBody);
[inline-code-end]