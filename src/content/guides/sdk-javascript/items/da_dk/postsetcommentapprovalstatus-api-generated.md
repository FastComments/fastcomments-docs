## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| approved | boolean | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Svar

Returns: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentApprovedResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'postSetCommentApprovalStatus Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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