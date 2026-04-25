## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| createTenantPackageBody | CreateTenantPackageBody | Ναι |  |

## Απόκριση

Επιστρέφει: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackage200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme-corp_001";
const createTenantPackageBody: CreateTenantPackageBody = {
  name: "Acme Standard Package",
  description: "Default package for Acme Corp comments with moderation and SSO enabled",
  enabled: true,
  maxCommentsPerThread: 500,
  voteStyle: "thumbs",
  gifRating: "PG-13",
  tosConfig: { enabled: true, url: "https://acme.example.com/terms" } // προαιρετική παράμετρος ως παράδειγμα
};
const result: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
[inline-code-end]

---