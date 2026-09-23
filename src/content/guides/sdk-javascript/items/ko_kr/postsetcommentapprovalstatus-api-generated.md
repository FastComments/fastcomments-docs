## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| approved | boolean | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## 응답

반환: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentApprovedResponse.ts)

## 예시

[inline-code-attrs-start title = 'postSetCommentApprovalStatus 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_001";
  const commentId: string = "comment_123";
  const approved: boolean = false;
  const broadcastId: string = "broadcast_456";
  const sso: string = "sso_789";

  const result: SetCommentApprovedResponse = await postSetCommentApprovalStatus(
    tenantId,
    commentId,
    approved,
    broadcastId,
    sso
  );

  const minimalResult: SetCommentApprovedResponse = await postSetCommentApprovalStatus(
    tenantId,
    commentId
  );
})();
[inline-code-end]