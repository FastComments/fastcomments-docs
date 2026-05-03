## פרמטרים

| שם | סוג | נדרש | תיאור |
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
const translationsBase: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread");
const translationsSpanishFullIds: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread", "es-ES", true);
[inline-code-end]

---