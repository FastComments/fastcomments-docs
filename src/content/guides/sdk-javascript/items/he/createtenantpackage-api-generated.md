## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| createTenantPackageBody | CreateTenantPackageBody | כן |  |

## תגובה

מחזיר: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackage200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-createTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7890";
const createTenantPackageBody: CreateTenantPackageBody = {
  packageName: "Growth Plan",
  maxSeats: 2500,
  features: {
    moderation: true,
    analytics: true,
    sso: { enabled: true, provider: "saml" }
  },
  billing: { interval: "monthly", priceCents: 19900 },
  // פרמטר אופציונלי מוצג: notes אינו נדרש אך סופק
  notes: "Onboarding bundle with priority support"
};
const result: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
[inline-code-end]