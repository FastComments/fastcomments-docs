## Parámetros

| Nombre | Tipo | Requerido | Descripción |
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
const translationsBase: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread");
const translationsSpanishFullIds: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread", "es-ES", true);
[inline-code-end]

---