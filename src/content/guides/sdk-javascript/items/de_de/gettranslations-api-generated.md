## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| namespace | string | Ja |  |
| component | string | Ja |  |
| locale | string | Nein |  |
| useFullTranslationIds | boolean | Nein |  |

## Antwort

Gibt zurück: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getTranslations Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsBase: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread");
const translationsSpanishFullIds: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread", "es-ES", true);
[inline-code-end]

---