## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| createCommentParams | CreateCommentParams | Tak |  |
| isLive | boolean | Nie |  |
| doSpamCheck | boolean | Nie |  |
| sendEmails | boolean | Nie |  |
| populateNotifications | boolean | Nie |  |

## Odpowiedź

Zwraca: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APISaveCommentResponse.ts)

## Przykład

[inline-code-attrs-start title = 'saveComment Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
  true,   // czy na żywo
  true,   // sprawdź spam
  false,  // wyślij e-maile
  true    // wypełnij powiadomienia
);
[inline-code-end]

---