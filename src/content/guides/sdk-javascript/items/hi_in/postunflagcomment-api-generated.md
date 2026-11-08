## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| broadcastId | string | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'postUnFlagComment उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "comment_98765";
  const broadcastId: string = "broadcast_55555";
  const sso: string = "sso_token_abcde";

  const result1: APIEmptyResponse = await postUnFlagComment(tenantId, commentId);
  const result2: APIEmptyResponse = await postUnFlagComment(tenantId, commentId, broadcastId, sso);
}

runExample();
[inline-code-end]