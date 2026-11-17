## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| sso | string | Hayır |  |
| includeUserInfo | PageReactIncludeUserInfo | Hayır |  |

## Yanıt

Döndürür: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV1PageLikes.ts)

## Örnek

[inline-code-attrs-start title = 'getV1PageLikes Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';
const urlId: string = 'https://example.com/blog/my-post';

const result: GetV1PageLikes = await getV1PageLikes(tenantId, urlId, 'sso-token-xyz789', true);

console.log(result.status);
if (result.likes) {
  result.likes.forEach((like) => {
    console.log(like.userId, like.date);
  });
}
[inline-code-end]