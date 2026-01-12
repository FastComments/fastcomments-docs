## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`PinComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PinComment200Response.ts)

## Example

[inline-code-attrs-start title = 'pinComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_1278";
  const commentId: string = "cmt_84b3f";
  const broadcastId: string = "brd_live_20260112";
  const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fake.signature";

  const pinnedWithSso: PinComment200Response = await pinComment(tenantId, commentId, broadcastId, ssoToken);
  const pinnedWithoutSso: PinComment200Response = await pinComment(tenantId, commentId, broadcastId);
  console.log(pinnedWithSso, pinnedWithoutSso);
})();
[inline-code-end]
