## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateTenantPackageBody | UpdateTenantPackageBody | 예 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'updateTenantPackage 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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