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
(async () => {
  const namespace: string = 'user-management';
  const component: string = 'profile-settings';
  const locale: string = 'en-GB';
  const useFullTranslationIds: boolean = true;
  const resultWithLocale: GetTranslationsResponse = await getTranslations(namespace, component, locale, useFullTranslationIds);
  const resultDefault: GetTranslationsResponse = await getTranslations(namespace, component);
})();
[inline-code-end]
