## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| commentId | string | Oui |  |
| approved | boolean | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentApprovedResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de postSetCommentApprovalStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = 'cmt_4b8f2a1d';
const approved: boolean = true;
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoSignature.example';
const result: SetCommentApprovedResponse = await postSetCommentApprovalStatus(commentId, approved, sso);
[inline-code-end]

---