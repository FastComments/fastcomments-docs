## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| voteId | string | हाँ |  |
| urlId | string | हाँ |  |
| broadcastId | string | हाँ |  |
| editKey | string | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'deleteCommentVote उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_9876";
const voteId: string = "vote_555";
const urlId: string = "url_abcde";
const broadcastId: string = "brd_2023";
const editKey: string = "edit_abc123"; // वैकल्पिक
const ssoToken: string = "sso_token_xyz"; // वैकल्पिक

// केवल आवश्यक पैरामीटर और एक वैकल्पिक पैरामीटर के साथ कॉल करें
const deleteResult: VoteDeleteResponse = await deleteCommentVote(
  tenantId,
  commentId,
  voteId,
  urlId,
  broadcastId,
  editKey
);

// दोनों वैकल्पिक पैरामीटर के साथ कॉल करें
const deleteResultWithSSO: VoteDeleteResponse = await deleteCommentVote(
  tenantId,
  commentId,
  voteId,
  urlId,
  broadcastId,
  editKey,
  ssoToken
);
[inline-code-end]