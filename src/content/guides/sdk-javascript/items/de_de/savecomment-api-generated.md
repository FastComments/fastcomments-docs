## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createCommentParams | CreateCommentParams | Ja |  |
| isLive | boolean | Nein |  |
| doSpamCheck | boolean | Nein |  |
| sendEmails | boolean | Nein |  |
| populateNotifications | boolean | Nein |  |

## Antwort

Gibt zurück: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveComment200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'saveComment Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fastcomments-tenant-42";
const createCommentParams: CreateCommentParams = {
  threadId: "article-2026-06-0142",
  content: "Great write-up — I followed the migration steps and everything worked as described.",
  userId: "u_9c72b",
  userName: "Ava R.",
  userAvatarUrl: "https://cdn.example.com/avatars/u_9c72b.png",
  metadata: { platform: "web", locale: "en-US" }
};
const isLive: boolean = true;
const doSpamCheck: boolean = true;
const sendEmails: boolean = false;
const populateNotifications: boolean = true;
const result: SaveComment200Response = await saveComment(tenantId, createCommentParams, isLive, doSpamCheck, sendEmails, populateNotifications);
[inline-code-end]