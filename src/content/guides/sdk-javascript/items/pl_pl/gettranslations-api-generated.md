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

[inline-code-attrs-start title = 'Przykład użycia getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsFull: GetTranslationsResponse = await getTranslations("site-comments", "comment-form", "fr-FR", true);
const translationsDefault: GetTranslationsResponse = await getTranslations("admin-dashboard", "notification-center");
[inline-code-end]

---