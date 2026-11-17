מתרגם את התגובות הנתונות ללוקל של הווידג'ט. זמין רק כאשר כלל ההתאמה האישית של הווידג'ט המתאים מוגדר עם Comment Translation כ-Automatic או Manual.

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| commentTranslationsRequest | CommentTranslationsRequest | כן |  |
| sso | string | לא |  |
| locale | string | לא |  |

## תגובה

מחזיר: [`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CommentTranslationsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת translateComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';
const urlId: string = 'blog-post-42';
const sso: string = 'JWT_TOKEN_HERE';

const commentTranslationsRequest: CommentTranslationsRequest = {
  commentIds: ['c_98765', 'c_54321'],
  targetLocale: 'es'
};

const locale: string = 'es-ES';

const response: CommentTranslationsResponse = await translateComments(
  tenantId,
  urlId,
  commentTranslationsRequest,
  sso,
  locale
);

console.log(`Translated ${response.translations.length} comments`);
[inline-code-end]