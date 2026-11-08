## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| isFlagged | boolean | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'flagCommentPublic उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_42";
const commentId: string = "comment_1001";
const isFlagged: boolean = true;
const sso: string = "sso_user_5678";

const resultWithSso: APIEmptyResponse = await flagCommentPublic(tenantId, commentId, isFlagged, sso);
const resultWithoutSso: APIEmptyResponse = await flagCommentPublic(tenantId, commentId, false);
[inline-code-end]