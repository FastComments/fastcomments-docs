## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| approved | boolean | नहीं |  |
| broadcastId | string | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentApprovedResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'postSetCommentApprovalStatus उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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