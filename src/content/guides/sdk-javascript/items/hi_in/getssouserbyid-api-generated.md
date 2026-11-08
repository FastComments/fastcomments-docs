## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetSSOUserByIdAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUserByIdAPIResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getSSOUserById उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUser() {
  const tenantId: string = "tenant-987654321";
  const userId: string = "sso-user-abc123";
  const result: GetSSOUserByIdAPIResponse = await getSSOUserById(tenantId, userId);
  const ssoUser: APISSOUser = result.user;
}
[inline-code-end]