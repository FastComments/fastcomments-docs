## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| createCommentParams | Array<CreateCommentParams> | Oui |  |
| isLive | boolean | Non |  |
| doSpamCheck | boolean | Non |  |
| sendEmails | boolean | Non |  |
| populateNotifications | boolean | Non |  |

## Réponse

Renvoie: `Array<SaveComment200Response`

## Exemple

[inline-code-attrs-start title = 'Exemple de saveCommentsBulk'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-01';
const mentions1: CommentUserMentionInfo[] = [{ userId: 'user-123', displayName: 'Jane Doe' }];
const hashtags1: CommentUserHashTagInfo[] = [{ tag: 'typescript' }];
const createCommentParams: CreateCommentParams[] = [
  {
    content: 'Great insights on async/await patterns.',
    authorName: 'John Smith',
    authorEmail: 'john.smith@acme.com',
    externalId: 'comment-001',
    createdAt: '2026-03-25T10:15:00Z',
    userMentions: mentions1,
    userHashTags: hashtags1
  },
  {
    content: 'I prefer using Promise.all for bulk ops.',
    authorName: 'Emily Turner',
    authorEmail: 'emily.turner@acme.com',
    externalId: 'comment-002',
    createdAt: '2026-03-25T10:20:00Z'
  }
];
const result: SaveComment200Response[] = await saveCommentsBulk(tenantId, createCommentParams, true, true, false, true);
[inline-code-end]

---