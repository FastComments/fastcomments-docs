## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| namespace | string | Da |  |
| component | string | Da |  |
| locale | string | Ne |  |
| useFullTranslationIds | boolean | Ne |  |

## Odgovor

Vraća: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsDefault: GetTranslationsResponse = await getTranslations("payments", "checkout");
const translationsFrenchDetailed: GetTranslationsResponse = await getTranslations("payments", "checkout", "fr-FR", true);
[inline-code-end]

---