Брой харесвания за страница, по избор с потребителите, които са я харесали.

## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| includeUserInfo | PageReactIncludeUserInfo | Не |  |

## Отговор

Връща: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPageLikesAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getPageLikes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'a1b2c3d4e5f6';
const urlId: string = 'https://mysite.com/blog/best-coffee-2024';

// Основно извикване с задължителните параметри
const likesResponse: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId);
console.log(`Страницата има ${likesResponse.pageLikes.length} харесвания`);

// Включване на информация за потребителя за всяко харесване
const likesWithUsers: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId, 'true');
likesWithUsers.pageLikes.forEach((like: PageReactUser) => {
  console.log(`${like.name} хареса тази страница на ${like.date}`);
});
[inline-code-end]