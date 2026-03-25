## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| createCommentParams | CreateCommentParams | Oui |  |
| isLive | boolean | Non |  |
| doSpamCheck | boolean | Non |  |
| sendEmails | boolean | Non |  |
| populateNotifications | boolean | Non |  |

## Réponse

Renvoie : [`SaveComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveComment200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de saveComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_001';
const createCommentParams: CreateCommentParams = {
  content: 'Great article — helped me fix a production issue in minutes.',
  url: 'https://app.acme.com/blog/performance-tips',
  author: { name: 'Maya Chen', email: 'maya.chen@acme.com' },
  metadata: { locale: 'en-US', appVersion: '4.2.1' }
} as CreateCommentParams;
const isLive: boolean = true;
const doSpamCheck: boolean = true;
const sendEmails: boolean = false;
const populateNotifications: boolean = true;
const result: SaveComment200Response = await saveComment(tenantId, createCommentParams, isLive, doSpamCheck, sendEmails, populateNotifications);
[inline-code-end]

---