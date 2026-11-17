Liczba polubień strony, opcjonalnie z użytkownikami, którzy ją polubili.

## Parametry

| Nazwa | Typ | Wymagany | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| includeUserInfo | PageReactIncludeUserInfo | Nie |  |

## Odpowiedź

Zwraca: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPageLikesAPIResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getPageLikes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'a1b2c3d4e5f6';
const urlId: string = 'https://mysite.com/blog/best-coffee-2024';

// Podstawowe wywołanie z wymaganymi parametrami
const likesResponse: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId);
console.log(`Strona ma ${likesResponse.pageLikes.length} polubień`);

// Dołącz informacje o użytkowniku dla każdego polubienia
const likesWithUsers: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId, 'true');
likesWithUsers.pageLikes.forEach((like: PageReactUser) => {
  console.log(`${like.name} polubił tę stronę ${like.date}`);
});
[inline-code-end]