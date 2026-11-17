Oversætter de givne kommentarer til widgettens lokalitet. Kun tilgængelig, når den tilsvarende widgettilpasningsregel har Comment Translation sat til Automatic eller Manual.

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| commentTranslationsRequest | CommentTranslationsRequest | Ja |  |
| sso | string | Nej |  |
| locale | string | Nej |  |

## Svar

Returnerer: [`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CommentTranslationsResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'translateComments Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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