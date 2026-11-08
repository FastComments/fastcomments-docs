## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | नहीं |  |
| externalId | string | नहीं |  |
| eventType | string | नहीं |  |
| type | string | नहीं |  |
| domain | string | नहीं |  |
| attemptCountGT | number | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCountResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getPendingWebhookEventCount उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_42";
  const commentId: string = "comment_1001";
  const eventType: string = "comment.deleted";
  const domain: string = "myblog.com";

  const result: GetPendingWebhookEventCountResponse = await getPendingWebhookEventCount(
    tenantId,
    commentId,
    undefined,
    eventType,
    undefined,
    domain
  );

  console.log(result);
}

runExample();
[inline-code-end]