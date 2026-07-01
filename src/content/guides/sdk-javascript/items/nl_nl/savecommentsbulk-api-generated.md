## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| createCommentParams | Array<CreateCommentParams> | Ja |  |
| isLive | boolean | Nee |  |
| doSpamCheck | boolean | Nee |  |
| sendEmails | boolean | Nee |  |
| populateNotifications | boolean | Nee |  |

## Respons

Retourneert: `Array<SaveCommentsBulkResponse`

## Voorbeeld

[inline-code-attrs-start title = 'saveCommentsBulk Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
  undefined  // populateNotifications (standaard gebruiken)
);
[inline-code-end]