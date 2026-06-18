## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createTenantPackageBody | CreateTenantPackageBody | 是 |  |

## 响应

返回: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackage200Response.ts)

## 示例

[inline-code-attrs-start title = 'createTenantPackage 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
  // 示例可选参数：notes 非必填，但已提供
  notes: "Onboarding bundle with priority support"
};
const result: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
[inline-code-end]