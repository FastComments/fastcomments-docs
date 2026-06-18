## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| namespace | string | Ja |  |
| component | string | Ja |  |
| locale | string | Nee |  |
| useFullTranslationIds | boolean | Nee |  |

## Respons

Retourneert: [`GetTranslations200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslations200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getTranslations Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const namespaceName: string = 'site-ui';
const componentName: string = 'comment-widget';
const locale: string = 'fr-FR';
const useFullTranslationIds: boolean = true;
const translationsWithLocale: GetTranslations200Response = await getTranslations(namespaceName, componentName, locale, useFullTranslationIds);
const translationsDefault: GetTranslations200Response = await getTranslations(namespaceName, componentName);
[inline-code-end]

---