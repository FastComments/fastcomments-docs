## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| approved | boolean | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentApprovedResponse.ts)

## 예제

[inline-code-attrs-start title = 'postSetCommentApprovalStatus 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = 'cmt_4b8f2a1d';
const approved: boolean = true;
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoSignature.example';
const result: SetCommentApprovedResponse = await postSetCommentApprovalStatus(commentId, approved, sso);
[inline-code-end]

---