## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Yes |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад updateTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";
const packageId: string = "pkg_3a2b1c";

const updateBody: UpdateTenantPackageBody = {
  // необов’язкові поля можна опускати або включати за потреби
  // newPackageName?: string;
  // renewalDate?: string;
};

const result: APIEmptyResponse = await updateTenantPackage(tenantId, packageId, updateBody);
[inline-code-end]

---