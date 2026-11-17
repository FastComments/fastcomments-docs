Broj lajkova za stranicu, opcionalno s korisnicima koji su je lajkali.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| includeUserInfo | PageReactIncludeUserInfo | Ne |  |

## Odgovor

Vraća: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPageLikesAPIResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer za getPageLikes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'a1b2c3d4e5f6';
const urlId: string = 'https://mysite.com/blog/best-coffee-2024';

// Osnovni poziv s obaveznim parametrima
const likesResponse: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId);
console.log(`Stranica ima ${likesResponse.pageLikes.length} lajkova`);

// Uključi informacije o korisniku za svaki lajk
const likesWithUsers: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId, 'true');
likesWithUsers.pageLikes.forEach((like: PageReactUser) => {
  console.log(`${like.name} je lajkao/la ovu stranicu ${like.date}`);
});
[inline-code-end]