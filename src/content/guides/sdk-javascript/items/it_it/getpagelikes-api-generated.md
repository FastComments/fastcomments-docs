Numero di like di una pagina, opzionalmente con gli utenti che hanno messo like.

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| includeUserInfo | PageReactIncludeUserInfo | No |  |

## Risposta

Restituisce: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPageLikesAPIResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getPageLikes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'a1b2c3d4e5f6';
const urlId: string = 'https://mysite.com/blog/best-coffee-2024';

// Chiamata base con i parametri obbligatori
const likesResponse: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId);
console.log(`Page has ${likesResponse.pageLikes.length} likes`);

// Includi le informazioni utente per ogni like
const likesWithUsers: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId, 'true');
likesWithUsers.pageLikes.forEach((like: PageReactUser) => {
  console.log(`${like.name} liked this page on ${like.date}`);
});
[inline-code-end]