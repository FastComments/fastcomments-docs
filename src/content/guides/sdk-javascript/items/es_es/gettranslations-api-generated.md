## Parámetros

| Name | Type | Requerido | Descripción |
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
const translationsDefault: GetTranslationsResponse = await getTranslations("payments", "checkout");
const translationsFrenchDetailed: GetTranslationsResponse = await getTranslations("payments", "checkout", "fr-FR", true);
[inline-code-end]

---