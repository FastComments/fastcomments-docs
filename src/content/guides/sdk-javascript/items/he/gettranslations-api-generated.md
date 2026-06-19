## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| namespace | string | כן |  |
| component | string | כן |  |
| locale | string | לא |  |
| useFullTranslationIds | boolean | לא |  |

## תשובה

מחזיר: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsFull: GetTranslationsResponse = await getTranslations("site-comments", "comment-form", "fr-FR", true);
const translationsDefault: GetTranslationsResponse = await getTranslations("admin-dashboard", "notification-center");
[inline-code-end]

---