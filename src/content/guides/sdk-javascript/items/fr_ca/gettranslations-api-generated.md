---
## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| namespace | string | Oui |  |
| component | string | Oui |  |
| locale | string | Non |  |
| useFullTranslationIds | boolean | Non |  |

## Réponse

Renvoie: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsFull: GetTranslationsResponse = await getTranslations("site-comments", "comment-form", "fr-FR", true);
const translationsDefault: GetTranslationsResponse = await getTranslations("admin-dashboard", "notification-center");
[inline-code-end]

---