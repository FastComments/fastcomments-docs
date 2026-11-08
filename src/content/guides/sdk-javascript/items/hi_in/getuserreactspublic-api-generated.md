## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| postIds | Array<string> | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UserReactsResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getUserReactsPublic उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const postIds: string[] = ["post_abc", "post_def"];
const ssoToken: string = "sso_token_987";

const fullResponse: UserReactsResponse = await getUserReactsPublic(tenantId, postIds, ssoToken);
const minimalResponse: UserReactsResponse = await getUserReactsPublic(tenantId);
[inline-code-end]