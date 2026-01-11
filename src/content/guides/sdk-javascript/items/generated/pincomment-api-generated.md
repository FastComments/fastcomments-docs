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
  const tenantId: string = "tenant_42f7a1";
  const commentId: string = "cmt-5e8d3c9a";
  const broadcastId: string = "bcast-20251122-01";
  const sso: string | undefined = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example_signature";
  const pinned: PinComment200Response = await pinComment(tenantId, commentId, broadcastId, sso);
  console.log(pinned);
})();
[inline-code-end]
