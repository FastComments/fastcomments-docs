## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| namespace | string | Da |  |
| component | string | Da |  |
| locale | string | Ne |  |
| useFullTranslationIds | boolean | Ne |  |

## Odgovor

Vrne: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsBase: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread");
const translationsSpanishFullIds: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread", "es-ES", true);
[inline-code-end]

---