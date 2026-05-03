## Parametreler

| Name | Type | Gerekli | Açıklama |
|------|------|----------|-------------|
| namespace | string | Evet |  |
| component | string | Evet |  |
| locale | string | Hayır |  |
| useFullTranslationIds | boolean | Hayır |  |

## Yanıt

Dönüş değeri: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getTranslations Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsBase: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread");
const translationsSpanishFullIds: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread", "es-ES", true);
[inline-code-end]

---