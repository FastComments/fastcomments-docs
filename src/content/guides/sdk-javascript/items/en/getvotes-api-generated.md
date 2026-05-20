## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Response

Returns: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotes200Response.ts)

## Example

[inline-code-attrs-start title = 'getVotes Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadVotes(): Promise<void> {
  const tenantId: string = 'acme-enterprises';
  const urlIdSuffix: string | undefined = 'q2-promo';
  const urlId: string = urlIdSuffix ? `promo-${urlIdSuffix}` : 'promo-default';
  const votes: GetVotes200Response = await getVotes(tenantId, urlId);
  void votes;
}
[inline-code-end]
