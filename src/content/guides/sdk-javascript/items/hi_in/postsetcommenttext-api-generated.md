## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| setCommentTextParams | SetCommentTextParams | हाँ |  |
| broadcastId | string | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentTextResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'postSetCommentText उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_9876";
const setCommentTextParams: SetCommentTextParams = {
  text: "Revised comment after user edit.",
  format: "plain"
};
const broadcastId: string = "broadcast_2023_09";

const response: SetCommentTextResponse = await postSetCommentText(
  tenantId,
  commentId,
  setCommentTextParams,
  broadcastId
);

console.log(response);
[inline-code-end]