## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| namespace | string | Evet |  |
| component | string | Evet |  |
| locale | string | Hayır |  |
| useFullTranslationIds | boolean | Hayır |  |

## Yanıt

Döndürür: [`GetTranslations200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslations200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getTranslations Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const namespaceName: string = 'site-ui';
const componentName: string = 'comment-widget';
const locale: string = 'fr-FR';
const useFullTranslationIds: boolean = true;
const translationsWithLocale: GetTranslations200Response = await getTranslations(namespaceName, componentName, locale, useFullTranslationIds);
const translationsDefault: GetTranslations200Response = await getTranslations(namespaceName, componentName);
[inline-code-end]