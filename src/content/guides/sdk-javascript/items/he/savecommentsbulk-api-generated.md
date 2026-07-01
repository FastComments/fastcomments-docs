## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| createCommentParams | Array<CreateCommentParams> | כן |  |
| isLive | boolean | לא |  |
| doSpamCheck | boolean | לא |  |
| sendEmails | boolean | לא |  |
| populateNotifications | boolean | לא |  |

## תגובה

מחזיר: `Array<SaveCommentsBulkResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמת saveCommentsBulk'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const bulkComments: CreateCommentParams[] = [
  {
    content: "Welcome to the new forum thread!",
    authorId: "user_42",
    mentions: [{ userId: "user_84", username: "alice" }],
    hashtags: [{ tag: "intro" }]
  },
  {
    content: "Please review the updated guidelines.",
    authorId: "moderator_1",
    mentions: [],
    hashtags: [{ tag: "guidelines" }, { tag: "update" }]
  }
];

const results: SaveCommentsBulkResponse[] = await saveCommentsBulk(
  tenantId,
  bulkComments,
  true,      // isLive
  false,     // doSpamCheck
  true,      // sendEmails
  undefined  // populateNotifications (בברירת מחדל)
);
[inline-code-end]