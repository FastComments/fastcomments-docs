## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Ответ

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример deleteTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_corp_987';
const packageId: string = 'pkg_pro_2026_01';
type DeleteOptions = { force?: boolean; notify?: boolean };
const options: DeleteOptions = { force: true }; // продемонстрированы необязательные параметры
const result: FlagCommentPublic200Response = await deleteTenantPackage(tenantId, packageId);
[inline-code-end]

---