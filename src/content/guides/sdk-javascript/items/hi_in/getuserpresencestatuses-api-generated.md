## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlIdWS | string | हाँ |  |
| userIds | string | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserPresenceStatusesResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getUserPresenceStatuses उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "my-tenant-42";
  const urlIdWS: string = "post-9876";
  const userIds: string = "alice,bob,carol";

  const presence: GetUserPresenceStatusesResponse = await getUserPresenceStatuses(tenantId, urlIdWS, userIds);
  console.log(presence);
})();
[inline-code-end]