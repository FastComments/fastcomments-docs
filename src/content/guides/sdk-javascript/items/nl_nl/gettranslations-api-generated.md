## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| namespace | string | Yes |  |
| component | string | Yes |  |
| locale | string | No |  |
| useFullTranslationIds | boolean | No |  |

## Respons

Retourneert: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getTranslations Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsFull: GetTranslationsResponse = await getTranslations("site-comments", "comment-form", "fr-FR", true);
const translationsDefault: GetTranslationsResponse = await getTranslations("admin-dashboard", "notification-center");
[inline-code-end]

---