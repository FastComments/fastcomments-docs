## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| postId | string | Ja |  |
| reactBodyParams | ReactBodyParams | Ja |  |
| isUndo | boolean | Nej |  |
| broadcastId | string | Nej |  |
| sso | string | Nej |  |

## Respons

Returnerer: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostPublic200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'reactFeedPostPublic Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-001';
const postId: string = 'feedpost_78901';
const reactBodyParams: ReactBodyParams = { reaction: 'like', emoji: '👍' };
const isUndo: boolean = false;
const broadcastId: string = 'broadcast_2026_06_15_01';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.signature';

const response: ReactFeedPostPublic200Response = await reactFeedPostPublic(
  tenantId,
  postId,
  reactBodyParams,
  isUndo,
  broadcastId,
  sso
);
[inline-code-end]

---