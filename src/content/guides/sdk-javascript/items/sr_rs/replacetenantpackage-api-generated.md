## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Yes |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Пример

[inline-code-attrs-start title = 'replaceTenantPackage Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function replacePackageDemo(): Promise<void> {
  const tenantId: string = "acme-corp-001";
  const packageId: string = "basic-plan-2023";
  const replaceBody: ReplaceTenantPackageBody = {
    newPackageId: "enterprise-plan-2024"
    // опционална поља могу бити додата овде ако је потребно
  };
  const response: APIEmptyResponse = await replaceTenantPackage(tenantId, packageId, replaceBody);
  console.log(response);
}
[inline-code-end]