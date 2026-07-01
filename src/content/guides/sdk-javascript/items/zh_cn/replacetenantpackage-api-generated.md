## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | 是 |  |

## 响应

返回：[`ReplaceTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReplaceTenantPackageResponse.ts)

## 示例

[inline-code-attrs-start title = 'replaceTenantPackage 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-corp-tenant-01";
    const packageId: string = "pkg-2024-annual";

    const config: CustomConfigParameters = {
        // 在此处添加自定义配置字段
    };

    const body: ReplaceTenantPackageBody = {
        name: "Enterprise Package",
        // 可选的自定义配置
        customConfig: config,
    };

    const response: ReplaceTenantPackageResponse = await replaceTenantPackage(tenantId, packageId, body);
    console.log(response);
})();
[inline-code-end]

---