## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| spam | boolean | नहीं |  |
| permNotSpam | boolean | नहीं |  |
| broadcastId | string | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'postSetCommentSpamStatus उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_42";
const commentId: string = "comment_1001";

const spam: boolean = true;
const permNotSpam: boolean = false;
const broadcastId: string = "broadcast_2023";
const sso: string = "sso_user_5678";

const resultFull: APIEmptyResponse = await postSetCommentSpamStatus(
  tenantId,
  commentId,
  spam,
  permNotSpam,
  broadcastId,
  sso
);

const resultMinimal: APIEmptyResponse = await postSetCommentSpamStatus(
  tenantId,
  commentId
);
[inline-code-end]