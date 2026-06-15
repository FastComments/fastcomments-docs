## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Ja |  |
| userId | string | Nej |  |
| anonUserId | string | Nej |  |

## Svar

Returnerer: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockCommentPublic200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'unBlockUserFromComment Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7b9c2a';
const id: string = 'comment_4f8e1d';
const unBlockFromCommentParams: UnBlockFromCommentParams = {
  reason: 'User submitted appeal and provided additional context',
  effectiveAt: new Date().toISOString()
};
const userId: string = 'user_92a3f6';
const result: UnBlockCommentPublic200Response = await unBlockUserFromComment(tenantId, id, unBlockFromCommentParams, userId);
[inline-code-end]

---