## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| namespace | string | Sí |  |
| component | string | Sí |  |
| locale | string | No |  |
| useFullTranslationIds | boolean | No |  |

## Respuesta

Devuelve: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsFull: GetTranslationsResponse = await getTranslations("site-comments", "comment-form", "fr-FR", true);
const translationsDefault: GetTranslationsResponse = await getTranslations("admin-dashboard", "notification-center");
[inline-code-end]

---