## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| createCommentParams | Array<CreateCommentParams> | Da |  |
| isLive | boolean | Ne |  |
| doSpamCheck | boolean | Ne |  |
| sendEmails | boolean | Ne |  |
| populateNotifications | boolean | Ne |  |

## Odgovor

Vraća: `Array<SaveCommentsBulkResponse`

## Primjer

[inline-code-attrs-start title = 'saveCommentsBulk Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
  undefined  // populateNotifications (using default)
);
[inline-code-end]