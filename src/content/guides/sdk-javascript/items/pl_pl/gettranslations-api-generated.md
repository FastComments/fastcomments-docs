## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| namespace | string | Tak |  |
| component | string | Tak |  |
| locale | string | Nie |  |
| useFullTranslationIds | boolean | Nie |  |

## Odpowiedź

Zwraca: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsBase: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread");
const translationsSpanishFullIds: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread", "es-ES", true);
[inline-code-end]