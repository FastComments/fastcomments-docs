Количество лайков для страницы, опционально с пользователями, которые её лайкнули.

## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| includeUserInfo | PageReactIncludeUserInfo | Нет |  |

## Ответ

Возвращает: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPageLikesAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getPageLikes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'a1b2c3d4e5f6';
const urlId: string = 'https://mysite.com/blog/best-coffee-2024';

// Базовый вызов с обязательными параметрами
const likesResponse: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId);
console.log(`У страницы ${likesResponse.pageLikes.length} лайков`);

// Включить информацию о пользователе для каждого лайка
const likesWithUsers: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId, 'true');
likesWithUsers.pageLikes.forEach((like: PageReactUser) => {
  console.log(`${like.name} лайкнул(а) эту страницу ${like.date}`);
});
[inline-code-end]