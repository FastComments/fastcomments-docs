## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Ja |  |

## Antwoord

Retourneert: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'updateTenantPackage Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8a4f1c9b';
const packageId: string = 'pkg_premium_v2';
const customConfig: CustomConfigParameters = { enableRichText: true, maxImagesPerComment: 5 };
const updateTenantPackageBody: UpdateTenantPackageBody = {
  name: 'Premium Moderation Package',
  enabled: true,
  description: 'Adds advanced spam rules, image moderation and priority support',
  customConfigParameters: customConfig
};
const result: FlagCommentPublic200Response = await updateTenantPackage(tenantId, packageId, updateTenantPackageBody);
[inline-code-end]

---