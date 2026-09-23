## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|-------------|------|
| tenantId | string | Так |  |
| id | string | Так |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Приклад

[inline-code-attrs-start title = 'deleteTenantPackage Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId: string = "tenant_12345";
let packageId: string = "pkg_98765";

const result: APIEmptyResponse = await deleteTenantPackage(tenantId, packageId);
[inline-code-end]

---