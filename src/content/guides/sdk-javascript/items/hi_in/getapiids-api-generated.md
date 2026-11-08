## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| textSearch | string | नहीं |  |
| byIPFromComment | string | नहीं |  |
| filters | string | नहीं |  |
| searchFilters | string | नहीं |  |
| afterId | string | नहीं |  |
| demo | boolean | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentIdsResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getApiIds उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const tenantId: string = "acme-corp";
  const textSearch: string = "offensive language";
  const byIPFromComment: string = "203.0.113.45";
  const filters: string = "status:unmoderated";
  const searchFilters: string = "author:alice";
  const afterId: string = "cmt_00123";
  const demo: boolean = true;
  const sso: string = "sso_9f8e7d6c";

  const fullResult: ModerationAPIGetCommentIdsResponse = await getApiIds(
    tenantId,
    textSearch,
    byIPFromComment,
    filters,
    searchFilters,
    afterId,
    demo,
    sso
  );

  const minimalResult: ModerationAPIGetCommentIdsResponse = await getApiIds(tenantId);
}
[inline-code-end]