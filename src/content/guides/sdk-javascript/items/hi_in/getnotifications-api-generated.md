## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| userId | string | नहीं |  |
| urlId | string | नहीं |  |
| fromCommentId | string | नहीं |  |
| viewed | boolean | नहीं |  |
| type | string | नहीं |  |
| skip | number | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationsResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getNotifications उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchNotifications(): Promise<void> {
  const tenantId: string = "tenant_42";
  const userId: string = "user_1001";
  const urlId: string = "url_2023";
  const viewed: boolean = true;
  const skip: number = 0;

  const notifications: GetNotificationsResponse = await getNotifications(
    tenantId,
    userId,
    urlId,
    undefined,
    viewed,
    undefined,
    skip
  );

  console.log(notifications);
}
[inline-code-end]