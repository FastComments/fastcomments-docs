## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Ναι |  |

## Απόκριση

Επιστρέφει: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα updateTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";
const packageId: string = "pkg_3a2b1c";

const updateBody: UpdateTenantPackageBody = {
  // προαιρετικά πεδία μπορούν να παραλειφθούν ή να συμπεριληφθούν ανάλογα με τις ανάγκες
  // newPackageName?: string;
  // renewalDate?: string;
};

const result: APIEmptyResponse = await updateTenantPackage(tenantId, packageId, updateBody);
[inline-code-end]

---