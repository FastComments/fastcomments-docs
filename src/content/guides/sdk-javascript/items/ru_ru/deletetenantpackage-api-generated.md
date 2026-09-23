## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Ответ

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример deleteTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId: string = "tenant_12345";
let packageId: string = "pkg_98765";

const result: APIEmptyResponse = await deleteTenantPackage(tenantId, packageId);
[inline-code-end]

---