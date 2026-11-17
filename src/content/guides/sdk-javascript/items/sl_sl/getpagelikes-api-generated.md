Število všečkov za stran, po možnosti skupaj z uporabniki, ki so všečkali.

## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| includeUserInfo | PageReactIncludeUserInfo | Ne |  |

## Odgovor

Vrne: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPageLikesAPIResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getPageLikes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'a1b2c3d4e5f6';
const urlId: string = 'https://mysite.com/blog/best-coffee-2024';

// Osnovni klic z obveznimi parametri
const likesResponse: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId);
console.log(`Stran ima ${likesResponse.pageLikes.length} všečkov`);

// Vključi podatke o uporabniku za vsak všeček
const likesWithUsers: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId, 'true');
likesWithUsers.pageLikes.forEach((like: PageReactUser) => {
  console.log(`${like.name} je to stran všečkal ${like.date}`);
});
[inline-code-end]