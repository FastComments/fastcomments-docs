## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| namespace | string | Da |  |
| component | string | Da |  |
| locale | string | Ne |  |
| useFullTranslationIds | boolean | Ne |  |

## Odgovor

Vraća: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsFull: GetTranslationsResponse = await getTranslations("site-comments", "comment-form", "fr-FR", true);
const translationsDefault: GetTranslationsResponse = await getTranslations("admin-dashboard", "notification-center");
[inline-code-end]

---