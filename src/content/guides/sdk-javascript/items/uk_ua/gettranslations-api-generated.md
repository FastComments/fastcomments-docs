## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| namespace | string | Так |  |
| component | string | Так |  |
| locale | string | Ні |  |
| useFullTranslationIds | boolean | Ні |  |

## Відповідь

Повертає: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsDefault: GetTranslationsResponse = await getTranslations("payments", "checkout");
const translationsFrenchDetailed: GetTranslationsResponse = await getTranslations("payments", "checkout", "fr-FR", true);
[inline-code-end]

---