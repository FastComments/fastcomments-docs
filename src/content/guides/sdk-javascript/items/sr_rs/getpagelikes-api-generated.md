Број лајкова за страницу, опционо са корисницима који су лајковали.

## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| includeUserInfo | PageReactIncludeUserInfo | Не |  |

## Одговор

Враћа: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPageLikesAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getPageLikes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'a1b2c3d4e5f6';
const urlId: string = 'https://mysite.com/blog/best-coffee-2024';

// Основни позив са обавезним параметрима
const likesResponse: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId);
console.log(`Страница има ${likesResponse.pageLikes.length} лајкова`);

// Укључи информације о кориснику за сваки лајк
const likesWithUsers: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId, 'true');
likesWithUsers.pageLikes.forEach((like: PageReactUser) => {
  console.log(`${like.name} је лајковао ову страницу ${like.date}`);
});
[inline-code-end]