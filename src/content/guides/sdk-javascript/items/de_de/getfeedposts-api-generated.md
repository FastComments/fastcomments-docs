req
tenantId
afterId

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| afterId | string | Nein |  |
| limit | number | Nein |  |
| tags | Array<string> | Nein |  |

## Antwort

Gibt zurück: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPosts200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getFeedPosts Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const initialPage: GetFeedPosts200Response = await getFeedPosts('tenant_9f1b3d', undefined, 20, ['sports', 'local']);
const nextPage: GetFeedPosts200Response = await getFeedPosts('tenant_9f1b3d', 'post_abc123', 20, ['sports', 'local']);
[inline-code-end]

---