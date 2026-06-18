## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| createTenantPackageBody | CreateTenantPackageBody | はい |  |

## レスポンス

戻り値: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackage200Response.ts)

## 例

[inline-code-attrs-start title = 'createTenantPackage の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
  // オプションのパラメータの例: notes は必須ではありませんが指定しています
  notes: "Onboarding bundle with priority support"
};
const result: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
[inline-code-end]