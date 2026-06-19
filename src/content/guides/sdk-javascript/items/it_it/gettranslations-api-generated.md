## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| namespace | string | Sì |  |
| component | string | Sì |  |
| locale | string | No |  |
| useFullTranslationIds | boolean | No |  |

## Risposta

Restituisce: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsFull: GetTranslationsResponse = await getTranslations("site-comments", "comment-form", "fr-FR", true);
const translationsDefault: GetTranslationsResponse = await getTranslations("admin-dashboard", "notification-center");
[inline-code-end]

---