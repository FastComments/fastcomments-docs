## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| namespace | string | Oui |  |
| component | string | Oui |  |
| locale | string | Non |  |
| useFullTranslationIds | boolean | Non |  |

## Réponse

Renvoie : [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsBase: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread");
const translationsSpanishFullIds: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread", "es-ES", true);
[inline-code-end]

---