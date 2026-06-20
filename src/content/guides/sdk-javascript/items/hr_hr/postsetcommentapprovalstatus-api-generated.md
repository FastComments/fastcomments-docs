## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| approved | boolean | No |  |
| sso | string | No |  |

## Odgovor

Vraća: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentApprovedResponse.ts)

## Primjer

[inline-code-attrs-start title = 'postSetCommentApprovalStatus Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = 'cmt_4b8f2a1d';
const approved: boolean = true;
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoSignature.example';
const result: SetCommentApprovedResponse = await postSetCommentApprovalStatus(commentId, approved, sso);
[inline-code-end]

---