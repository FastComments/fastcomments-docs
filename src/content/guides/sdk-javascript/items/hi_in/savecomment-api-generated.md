## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createCommentParams | CreateCommentParams | हाँ |  |
| isLive | boolean | नहीं |  |
| doSpamCheck | boolean | नहीं |  |
| sendEmails | boolean | नहीं |  |
| populateNotifications | boolean | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APISaveCommentResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'saveComment उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const commentParams: CreateCommentParams = {
  content: "This is a comment with a mention and a hashtag.",
  userId: "user_987",
  mentions: [
    { userId: "user_123", start: 27, end: 34 }
  ] as CommentUserMentionInfo[],
  hashtags: [
    { tag: "feedback", start: 45, end: 53 }
  ] as CommentUserHashTagInfo[],
  poll: {
    question: "Do you like this feature?",
    options: ["Yes", "No"]
  } as CommentPollInput,
};

const response: APISaveCommentResponse = await saveComment(
  tenantId,
  commentParams,
  true,   // isLive
  true,   // doSpamCheck
  false,  // sendEmails
  true    // populateNotifications
);
[inline-code-end]