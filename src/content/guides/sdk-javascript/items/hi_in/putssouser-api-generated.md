## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| updateAPISSOUserData | UpdateAPISSOUserData | हाँ |  |
| updateComments | boolean | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutSSOUserAPIResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'putSSOUser उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_abc123";
  const userId: string = "user_456def";

  const updateData: UpdateAPISSOUserData = {
    email: "jane.doe@example.com",
    displayName: "Jane Doe",
    isActive: true,
  };

  const response: PutSSOUserAPIResponse = await putSSOUser(tenantId, userId, updateData, true);
})();
[inline-code-end]