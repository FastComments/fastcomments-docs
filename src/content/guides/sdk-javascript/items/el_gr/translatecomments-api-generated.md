Μεταφράζει τα δοσμένα σχόλια στην τοπική ρύθμιση του widget. Διαθέσιμο μόνο όταν ο αντίστοιχος κανόνας προσαρμογής του widget έχει το Comment Translation ορισμένο σε Automatic ή Manual.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| commentTranslationsRequest | CommentTranslationsRequest | Ναι |  |
| sso | string | Όχι |  |
| locale | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CommentTranslationsResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα translateComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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