## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| value | string | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationUserSearchResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getSearchUsers उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runSearch() {
  const tenantId: string = "tenant-9876";
  const emailFragment: string = "jane";
  const ssoToken: string = "sso-token-456";

  const resultWithAll: ModerationUserSearchResponse = await getSearchUsers(tenantId, emailFragment, ssoToken);
  const resultWithTenantOnly: ModerationUserSearchResponse = await getSearchUsers(tenantId);
}
[inline-code-end]