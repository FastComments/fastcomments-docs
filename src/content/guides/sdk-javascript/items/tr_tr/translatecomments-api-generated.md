Verilen yorumları widget yereline çevirir. Yalnızca eşleşen widget özelleştirme kuralında Comment Translation ayarı Automatic veya Manual olarak belirlenmişse kullanılabilir.

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| commentTranslationsRequest | CommentTranslationsRequest | Evet |  |
| sso | string | Hayır |  |
| locale | string | Hayır |  |

## Yanıt

Döndürür: [`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CommentTranslationsResponse.ts)

## Örnek

[inline-code-attrs-start title = 'translateComments Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';
const urlId: string = 'blog-post-42';
const sso: string = 'JWT_TOKEN_HERE';

const commentTranslationsRequest: CommentTranslationsRequest = {
  commentIds: ['c_98765', 'c_54321'],
  targetLocale: 'es'
};

const locale: string = 'es-ES';

const response: CommentTranslationsResponse = await translateComments(
  tenantId,
  urlId,
  commentTranslationsRequest,
  sso,
  locale
);

console.log(`${response.translations.length} yorum çevrildi`);
[inline-code-end]