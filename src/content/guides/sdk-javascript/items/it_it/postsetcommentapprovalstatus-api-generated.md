---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| commentId | string | Sì |  |
| approved | boolean | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentApprovedResponse.ts)

## Esempio

[inline-code-attrs-start title = 'postSetCommentApprovalStatus Esempio'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = 'cmt_4b8f2a1d';
const approved: boolean = true;
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoSignature.example';
const result: SetCommentApprovedResponse = await postSetCommentApprovalStatus(commentId, approved, sso);
[inline-code-end]

---