---
## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Одговор

Враћа: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Примјер

[inline-code-attrs-start title = 'deleteEmailTemplate Примјер'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f4c9d1e';
const templateId: string = 'tmpl_welcome_2024-03';
const notifyAdmin: boolean | undefined = true; // примјер опционалног параметра

const result: FlagCommentPublic200Response = await deleteEmailTemplate(tenantId, templateId);
[inline-code-end]

---