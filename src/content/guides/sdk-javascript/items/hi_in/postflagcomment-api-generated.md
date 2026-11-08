## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| broadcastId | string | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'postFlagComment उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";

  // केवल आवश्यक पैरामीटर
  const result1: APIEmptyResponse = await postFlagComment(tenantId, commentId);

  // वैकल्पिक पैरामीटर सहित
  const broadcastId: string = "brd_54321";
  const sso: string = "user@example.com";
  const result2: APIEmptyResponse = await postFlagComment(tenantId, commentId, broadcastId, sso);

  console.log(result1, result2);
}
runExample();
[inline-code-end]