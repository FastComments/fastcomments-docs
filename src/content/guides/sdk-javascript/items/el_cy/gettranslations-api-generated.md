## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
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
const translationsBase: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread");
const translationsSpanishFullIds: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread", "es-ES", true);
[inline-code-end]

---