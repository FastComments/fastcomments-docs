## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| id | string | Так |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Так |  |

## Відповідь

Повертає: [`ReplaceTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReplaceTenantPackageResponse.ts)

## Приклад

[inline-code-attrs-start title = 'replaceTenantPackage Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-corp-tenant-01";
    const packageId: string = "pkg-2024-annual";

    const config: CustomConfigParameters = {
        // поля користувацької конфігурації тут
    };

    const body: ReplaceTenantPackageBody = {
        name: "Enterprise Package",
        // необов'язкова користувацька конфігурація
        customConfig: config,
    };

    const response: ReplaceTenantPackageResponse = await replaceTenantPackage(tenantId, packageId, body);
    console.log(response);
})();
[inline-code-end]