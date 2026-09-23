## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| approved | boolean | Nein |  |
| broadcastId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Returns: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentApprovedResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'postSetCommentApprovalStatus Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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