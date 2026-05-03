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
const translationsBase: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread");
const translationsSpanishFullIds: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread", "es-ES", true);
[inline-code-end]
