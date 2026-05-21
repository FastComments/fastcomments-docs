## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`GetDomainConfigs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfigs200Response.ts)

## Example

[inline-code-attrs-start title = 'getDomainConfigs Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async function run(): Promise<void> {
  const tenantId: string = 'd4f8c9e0-3b1a-4f2a-8c9d-1e2f3a4b5c6d';
  const options: { includeInactive?: boolean } = { includeInactive: true };
  const result: GetDomainConfigs200Response = await getDomainConfigs(tenantId, options);
  console.log(result);
})();
[inline-code-end]
