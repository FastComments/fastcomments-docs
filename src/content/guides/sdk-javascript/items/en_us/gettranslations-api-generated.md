## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| namespace | string | Yes |  |
| component | string | Yes |  |
| locale | string | No |  |
| useFullTranslationIds | boolean | No |  |

## Response

Returns: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Example

[inline-code-attrs-start title = 'getTranslations Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsFull: GetTranslationsResponse = await getTranslations("site-comments", "comment-form", "fr-FR", true);
const translationsDefault: GetTranslationsResponse = await getTranslations("admin-dashboard", "notification-center");
[inline-code-end]
