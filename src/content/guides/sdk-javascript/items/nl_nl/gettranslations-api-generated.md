## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| namespace | string | Ja |  |
| component | string | Ja |  |
| locale | string | Nee |  |
| useFullTranslationIds | boolean | Nee |  |

## Respons

Retourneert: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getTranslations Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsBase: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread");
const translationsSpanishFullIds: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread", "es-ES", true);
[inline-code-end]

---