## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| textSearch | string | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationSuggestResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getSearchSuggest उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-9f8b7c6d";
  const textSearch: string = "harassment";
  const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
  const suggestResponse: ModerationSuggestResponse = await getSearchSuggest(tenantId, textSearch, sso);
  const minimalResponse: ModerationSuggestResponse = await getSearchSuggest(tenantId);
})();
[inline-code-end]