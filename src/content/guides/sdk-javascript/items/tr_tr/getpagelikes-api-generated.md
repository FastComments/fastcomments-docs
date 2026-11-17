Bir sayfa için beğeni sayısı, isteğe bağlı olarak beğenen kullanıcılarla birlikte.

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| includeUserInfo | PageReactIncludeUserInfo | Hayır |  |

## Yanıt

Döndürür: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPageLikesAPIResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getPageLikes Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'a1b2c3d4e5f6';
const urlId: string = 'https://mysite.com/blog/best-coffee-2024';

// Zorunlu parametrelerle temel çağrı
const likesResponse: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId);
console.log(`Sayfada ${likesResponse.pageLikes.length} beğeni var`);

// Her beğeni için kullanıcı bilgilerini dahil et
const likesWithUsers: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId, 'true');
likesWithUsers.pageLikes.forEach((like: PageReactUser) => {
  console.log(`${like.name} bu sayfayı ${like.date} tarihinde beğendi`);
});
[inline-code-end]