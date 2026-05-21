---
## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| namespace | string | Ναι |  |
| component | string | Ναι |  |
| locale | string | Όχι |  |
| useFullTranslationIds | boolean | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsDefault: GetTranslationsResponse = await getTranslations("payments", "checkout");
const translationsFrenchDetailed: GetTranslationsResponse = await getTranslations("payments", "checkout", "fr-FR", true);
[inline-code-end]

---