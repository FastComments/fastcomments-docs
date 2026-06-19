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

Renvoie: `Array<SaveCommentsBulkResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple de saveCommentsBulk'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme_corp_tenant_12';
const createCommentParams: Array<CreateCommentParams> = [
  {
    content: 'Really helpful breakdown of the migration steps.',
    threadId: 'thread_2026_08',
    authorName: 'Maya Singh',
    authorEmail: 'maya.singh@startup.io',
    mentions: [{ userId: 'user_314', displayName: 'Leo Park' }],
    hashtags: [{ tag: 'migration' }],
    createdAt: '2026-06-19T12:00:00Z'
  }
];
const isLive: boolean = true;
const doSpamCheck: boolean = true;
const sendEmails: boolean = false;
const populateNotifications: boolean = true;
const responses: Array<SaveCommentsBulkResponse> = await saveCommentsBulk(tenantId, createCommentParams, isLive, doSpamCheck, sendEmails, populateNotifications);
[inline-code-end]

---