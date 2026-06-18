## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| usernameStartsWith | string | Нет |  |
| mentionGroupIds | Array<string> | Нет |  |
| sso | string | Нет |  |
| searchSection | SearchUsersSearchSectionEnum | Нет |  |

## Ответ

Возвращает: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SearchUsers200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример searchUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7890';
const urlId: string = 'news/2026-06-15-world-cup-final';
const usernameStartsWith: string = 'mar';
const mentionGroupIds: string[] = ['staff', 'trusted-commenters'];
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI0NTY3OCJ9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
const searchSection: SearchUsersSearchSectionEnum = SearchUsersSearchSectionEnum.Commenters;
const response: SearchUsers200Response = await searchUsers(tenantId, urlId, usernameStartsWith, mentionGroupIds, sso, searchSection);
[inline-code-end]

---