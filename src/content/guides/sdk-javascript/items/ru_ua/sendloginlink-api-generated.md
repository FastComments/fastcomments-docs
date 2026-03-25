## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| redirectURL | string | Нет |  |

## Ответ

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример sendLoginLink'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12a9f3b7";
const id: string = "user_84b2c7d1";
const redirectURL: string = "https://app.mycompany.com/welcome?ref=login_email";
const resultWithoutRedirect: FlagCommentPublic200Response = await sendLoginLink(tenantId, id);
const resultWithRedirect: FlagCommentPublic200Response = await sendLoginLink(tenantId, id, redirectURL);
[inline-code-end]

---