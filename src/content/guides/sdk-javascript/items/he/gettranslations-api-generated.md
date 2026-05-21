## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| namespace | string | כן |  |
| component | string | כן |  |
| locale | string | לא |  |
| useFullTranslationIds | boolean | לא |  |

## תגובה

מחזיר: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsDefault: GetTranslationsResponse = await getTranslations("payments", "checkout");
const translationsFrenchDetailed: GetTranslationsResponse = await getTranslations("payments", "checkout", "fr-FR", true);
[inline-code-end]