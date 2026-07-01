## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentIds | string | Yes |  |
| sso | string | No |  |

## Response

Επιστρέφει: [`CheckedCommentsForBlockedResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CheckedCommentsForBlockedResponse.ts)

## Example

[inline-code-attrs-start title = 'checkedCommentsForBlocked Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-123";
  const commentIds: string = "cmt_001,cmt_002";
  const ssoToken: string = "ssoTokenXYZ";

  const blockedCheck: CheckedCommentsForBlockedResponse = await checkedCommentsForBlocked(tenantId, commentIds);
  const blockedCheckWithSso: CheckedCommentsForBlockedResponse = await checkedCommentsForBlocked(tenantId, commentIds, ssoToken);
})();
[inline-code-end]

---