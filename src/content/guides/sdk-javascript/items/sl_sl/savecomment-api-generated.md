## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createCommentParams | CreateCommentParams | Da |  |
| isLive | boolean | Ne |  |
| doSpamCheck | boolean | Ne |  |
| sendEmails | boolean | Ne |  |
| populateNotifications | boolean | Ne |  |

## Odgovor

Vrne: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APISaveCommentResponse.ts)

## Primer

[inline-code-attrs-start title = 'saveComment Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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