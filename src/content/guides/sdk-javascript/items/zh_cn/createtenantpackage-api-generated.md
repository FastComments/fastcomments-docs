## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| createTenantPackageBody | CreateTenantPackageBody | 是 |  |

## 响应

返回: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackage200Response.ts)

## 示例

[inline-code-attrs-start title = 'createTenantPackage 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f3b1a9c";
const tenantPackage: TenantPackage = { id: "pkg_001", name: "Premium Plan", seats: 100 };
const customConfig: CustomConfigParameters = { enableImages: true, maxImageSizeMb: 10 };
const createTenantPackageBody: CreateTenantPackageBody = {
  packageName: "Premium Plus",
  tenantPackage,
  customConfig,
  notes: "Enable advanced moderation and image uploads" // 可选参数示例
};
const result: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
[inline-code-end]