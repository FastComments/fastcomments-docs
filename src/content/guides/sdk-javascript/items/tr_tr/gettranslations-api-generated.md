## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| namespace | string | Evet |  |
| component | string | Evet |  |
| locale | string | Hayır |  |
| useFullTranslationIds | boolean | Hayır |  |

## Yanıt

Döndürür: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getTranslations Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsFull: GetTranslationsResponse = await getTranslations("site-comments", "comment-form", "fr-FR", true);
const translationsDefault: GetTranslationsResponse = await getTranslations("admin-dashboard", "notification-center");
[inline-code-end]