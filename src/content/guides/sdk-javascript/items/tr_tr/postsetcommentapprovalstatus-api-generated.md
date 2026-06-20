## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Evet |  |
| approved | boolean | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentApprovedResponse.ts)

## Örnek

[inline-code-attrs-start title = 'postSetCommentApprovalStatus Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = 'cmt_4b8f2a1d';
const approved: boolean = true;
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoSignature.example';
const result: SetCommentApprovedResponse = await postSetCommentApprovalStatus(commentId, approved, sso);
[inline-code-end]

---