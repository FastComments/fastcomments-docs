## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| textSearch | string | नहीं |  |
| byIPFromComment | string | नहीं |  |
| filter | string | नहीं |  |
| searchFilters | string | नहीं |  |
| demo | boolean | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICountCommentsResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getCount उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const basicCount: ModerationAPICountCommentsResponse = await getCount(tenantId);

const detailedCount: ModerationAPICountCommentsResponse = await getCount(
  tenantId,
  "spam",
  "192.168.1.100",
  "status:pending",
  "user:john",
  true,
  "sso_token_abc"
);
[inline-code-end]