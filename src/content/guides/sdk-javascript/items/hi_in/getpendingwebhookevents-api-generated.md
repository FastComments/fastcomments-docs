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
| skip | number | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventsResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getPendingWebhookEvents उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "123e4567-e89b-12d3-a456-426614174000";
const commentId: string = "cmt_987654321";
const externalId: string = "ext_abc123";
const eventType: string = "comment_created";
const type: string = "outbound";
const domain: string = "myblog.com";
const attemptCountGT: number = 2;
const skip: number = 10;

const pendingEvents: GetPendingWebhookEventsResponse = await getPendingWebhookEvents(
  tenantId,
  commentId,
  externalId,
  eventType,
  type,
  domain,
  attemptCountGT,
  skip
);
[inline-code-end]