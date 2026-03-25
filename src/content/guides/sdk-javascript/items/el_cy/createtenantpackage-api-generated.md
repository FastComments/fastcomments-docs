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
const tenantId: string = 'tenant_acme_001';
const createTenantPackageBody: CreateTenantPackageBody = {
  packageName: 'Standard Moderation',
  description: 'Suitable for small-to-medium sites: basic moderation, spam rules, and analytics',
  maxCommentsPerMinute: 50,
  allowAnonymousComments: false, // προαιρετική παράμετρος που παρέχεται
  // προαιρετικά πεδία παραλείπονται: π.χ. προηγμένοι κανόνες διαχείρισης σχολίων, προσαρμοσμένο CSS
  customConfigParameters: {
    enableProfanityFilter: true,
    imageContentProfanityLevel: 'medium' // ενδεικτική τιμή· χρησιμοποιεί το σχήμα CustomConfigParameters
  }
};
const response: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
console.log(response);
[inline-code-end]

---