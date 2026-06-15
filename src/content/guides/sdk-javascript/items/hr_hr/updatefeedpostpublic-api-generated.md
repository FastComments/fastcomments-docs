## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| postId | string | Da |  |
| updateFeedPostParams | UpdateFeedPostParams | Da |  |
| broadcastId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostPublic200Response.ts)

## Primjer

[inline-code-attrs-start title = 'updateFeedPostPublic Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_01';
const postId: string = 'post_20260615_001';
const updateFeedPostParams: UpdateFeedPostParams = {
  title: 'Weekly Update: Product Launch',
  content: 'We shipped the 2.0 release today — highlights and links below.',
  media: [{ url: 'https://cdn.acme.com/releases/launch.jpg', type: 'image' }],
  tags: ['release', 'product'],
  isPublic: true
};
const broadcastId: string = 'broadcast_live_42';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const result: CreateFeedPostPublic200Response = await updateFeedPostPublic(tenantId, postId, updateFeedPostParams, broadcastId, sso);
[inline-code-end]

---