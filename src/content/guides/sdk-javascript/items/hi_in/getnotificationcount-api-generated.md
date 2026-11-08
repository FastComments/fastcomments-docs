## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| userId | string | नहीं |  |
| urlId | string | नहीं |  |
| fromCommentId | string | नहीं |  |
| viewed | boolean | नहीं |  |
| type | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCountResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getNotificationCount उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const userId: string = "user-12345";
const urlId: string = "https://app.example.com/dashboard";
const fromCommentId: string = "cmt-9876";
const viewed: boolean = false;
const type: string = "reply";

const notificationCount: GetNotificationCountResponse = await getNotificationCount(
  tenantId,
  userId,
  urlId,
  fromCommentId,
  viewed,
  type
);
[inline-code-end]