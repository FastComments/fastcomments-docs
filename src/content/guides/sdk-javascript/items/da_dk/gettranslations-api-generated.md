## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| namespace | string | Ja |  |
| component | string | Ja |  |
| locale | string | Nej |  |
| useFullTranslationIds | boolean | Nej |  |

## Respons

Returnerer: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsFull: GetTranslationsResponse = await getTranslations("site-comments", "comment-form", "fr-FR", true);
const translationsDefault: GetTranslationsResponse = await getTranslations("admin-dashboard", "notification-center");
[inline-code-end]

---