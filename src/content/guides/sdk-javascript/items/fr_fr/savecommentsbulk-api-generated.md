## Parameters

| Nom | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenantId | string | Oui |  |
| createCommentParams | Array<CreateCommentParams> | Oui |  |
| isLive | boolean | Non |  |
| doSpamCheck | boolean | Non |  |
| sendEmails | boolean | Non |  |
| populateNotifications | boolean | Non |  |

## Response

Retourne : `Array<SaveCommentsBulkResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple saveCommentsBulk'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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