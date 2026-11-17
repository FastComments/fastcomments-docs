Translates the given comments into the widget locale. Only available when the matching widget
customization rule has Comment Translation set to Automatic or Manual.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| commentTranslationsRequest | CommentTranslationsRequest | Yes |  |
| sso | string | No |  |
| locale | string | No |  |

## Response

Returns: [`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CommentTranslationsResponse.ts)

## Example

[inline-code-attrs-start title = 'translateComments Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
