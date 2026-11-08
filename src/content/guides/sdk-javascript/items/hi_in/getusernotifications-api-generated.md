## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | नहीं |  |
| pageSize | number | नहीं |  |
| afterId | string | नहीं |  |
| includeContext | boolean | नहीं |  |
| afterCreatedAt | number | नहीं |  |
| unreadOnly | boolean | नहीं |  |
| dmOnly | boolean | नहीं |  |
| noDm | boolean | नहीं |  |
| includeTranslations | boolean | नहीं |  |
| includeTenantNotifications | boolean | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMyNotificationsResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getUserNotifications उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchNotifications(): Promise<void> {
  const tenantId: string = "acme-corp";
  const urlId: string = "article-42";
  const pageSize: number = 20;
  const includeContext: boolean = true;
  const unreadOnly: boolean = true;

  const notifications: GetMyNotificationsResponse = await getUserNotifications(
    tenantId,
    urlId,
    pageSize,
    undefined,
    includeContext,
    undefined,
    unreadOnly
  );

  console.log(notifications);
}
[inline-code-end]