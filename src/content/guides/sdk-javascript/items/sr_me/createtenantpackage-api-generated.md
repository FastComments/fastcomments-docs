## Параметри

| Име | Тип | Потребно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createTenantPackageBody | CreateTenantPackageBody | Да |  |

## Одговор

Враћа: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackage200Response.ts)

## Пример

[inline-code-attrs-start title = 'createTenantPackage Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
  // пример опционалног параметра: notes није обавезан, али је наведен
  notes: "Onboarding bundle with priority support"
};
const result: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
[inline-code-end]

---