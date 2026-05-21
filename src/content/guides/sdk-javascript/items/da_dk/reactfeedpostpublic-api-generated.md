## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| postId | string | Ja |  |
| reactBodyParams | ReactBodyParams | Ja |  |
| isUndo | boolean | Nej |  |
| broadcastId | string | Nej |  |
| urlId | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostPublic200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'reactFeedPostPublic Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "global-markets";
const postId: string = "8e2c3f9a-4b6d-4f1a-9c2d-e8a1b2c3d4e5";
const reactBodyParams: ReactBodyParams = { reactionType: "like", clientApp: "web-ui", timestamp: new Date().toISOString() };
const isUndo: boolean = false;
const broadcastId: string = "broadcast-2026-05-20";
const urlId: string = "feed-post-8e2c";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fake.payload";

const result: ReactFeedPostPublic200Response = await reactFeedPostPublic(tenantId, postId, reactBodyParams, isUndo, broadcastId, urlId, sso);
[inline-code-end]

---