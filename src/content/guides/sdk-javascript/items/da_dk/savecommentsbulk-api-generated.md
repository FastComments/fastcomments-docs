## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createCommentParams | Array<CreateCommentParams> | Ja |  |
| isLive | boolean | Nej |  |
| doSpamCheck | boolean | Nej |  |
| sendEmails | boolean | Nej |  |
| populateNotifications | boolean | Nej |  |

## Svar

Returnerer: `Array<SaveComment200Response`

## Eksempel

[inline-code-attrs-start title = 'saveCommentsBulk Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_42a1b7";
const mentions: CommentUserMentionInfo[] = [{ userId: "user_2b9", displayName: "Alex Chen" }];
const hashtags: CommentUserHashTagInfo[] = [{ tag: "performance" }];
const createCommentParams: CreateCommentParams[] = [
  {
    content: "Thanks for the detailed article — the alternative approach worked for me.",
    authorId: "user_8f3c2",
    authorName: "Maya Patel",
    authorEmail: "maya.patel@example.com",
    url: "/articles/optimizing-ts-performance",
    createdAt: new Date().toISOString(),
    mentions,
    hashtags
  }
];
const isLive: boolean = true;
const doSpamCheck: boolean = false;
const sendEmails: boolean = true;
const populateNotifications: boolean = true;
const result: Array<SaveComment200Response> = await saveCommentsBulk(tenantId, createCommentParams, isLive, doSpamCheck, sendEmails, populateNotifications);
[inline-code-end]