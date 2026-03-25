## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Tak |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublic200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład blockFromCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "site_7f9b2e";
const commentId: string = "comment_2026-03-25_001";
const publicBlockFromCommentParams: PublicBlockFromCommentParams = {
  reason: "Repeated harassment and targeted abuse",
  blockDurationDays: 90,
  includeHistory: true,
  notifyModeratorTeam: true
};
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fakePayload.signature";
const result: BlockFromCommentPublic200Response = await blockFromCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
[inline-code-end]

---