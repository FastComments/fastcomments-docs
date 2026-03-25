---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| postIds | Array<string> | Ja |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`GetFeedPostsStats200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsStats200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getFeedPostsStats Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9b2f1c4a';
const postIds: Array<string> = [
  '8f14e45f-ea82-4c7a-b6b2-1a2b3c4d5e6f',
  'd0e1f2a3-b4c5-6d7e-8f90-1234567890ab'
];
const sso: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.signature';
const statsWithoutSSO: GetFeedPostsStats200Response = await getFeedPostsStats(tenantId, postIds);
const statsWithSSO: GetFeedPostsStats200Response = await getFeedPostsStats(tenantId, postIds, sso);
[inline-code-end]

---