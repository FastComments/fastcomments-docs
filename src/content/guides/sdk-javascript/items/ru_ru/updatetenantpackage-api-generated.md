---
## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Да |  |

## Ответ

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример updateTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4b7c9a2f";
const id: string = "pkg_91f2d3b8";
const updateTenantPackageBody: UpdateTenantPackageBody = {
  planId: "business_annual",
  seats: 50,
  autoRenew: true,
  couponCode: "WELCOME2025" // необязательный параметр (демонстрация)
};
const result: APIEmptyResponse = await updateTenantPackage(tenantId, id, updateTenantPackageBody);
[inline-code-end]

---