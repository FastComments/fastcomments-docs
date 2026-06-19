## Параметры

| Название | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| usernameStartsWith | string | Нет |  |
| mentionGroupIds | Array<string> | Нет |  |
| sso | string | Нет |  |
| searchSection | SearchUsersSearchSectionEnum | Нет |  |

## Ответ

Возвращает: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SearchUsersResult.ts)

## Пример

[inline-code-attrs-start title = 'searchUsers Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_9f7b3a';
  const urlId: string = 'news/2026/fastcomments-release';
  const usernameStartsWith: string = 'ann';
  const mentionGroupIds: string[] = ['editors', 'contributors'];
  const sso: string = 'google-oauth2';
  const searchSection: SearchUsersSearchSectionEnum = SearchUsersSearchSectionEnum.Mentions;
  const result: SearchUsersResult = await searchUsers(tenantId, urlId, usernameStartsWith, mentionGroupIds, sso, searchSection);
  console.log(result);
})();
[inline-code-end]