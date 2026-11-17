## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| skip | number | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackagesResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getTenantPackages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5ad8b1e4-8c1b-4f2a-9d3e-7f6a1b2c3d4e';

// Λήψη της πρώτης σελίδας των πακέτων του tenant
const response: GetTenantPackagesResponse = await getTenantPackages(tenantId);
const firstPackage: TenantPackage | undefined = response.packages?.[0];
if (firstPackage?.id) {
  console.log(`First package: ${firstPackage.id} - active: ${firstPackage.active}`);
}

// Λήψη της επόμενης σελίδας με χρήση του skip
const skip: number = 10;
const nextPage: GetTenantPackagesResponse = await getTenantPackages(tenantId, skip);
const status: APIStatus = nextPage.status;
console.log(`Fetched ${nextPage.packages?.length ?? 0} packages, status: ${status}`);
[inline-code-end]