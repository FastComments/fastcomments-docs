## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerator200Response.ts)

## Example

[inline-code-attrs-start title = 'getModerator Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async function run(): Promise<void> {
  const tenantId: string = 'tenant_company_82';
  const id: string = 'moderator_4932';
  const response: GetModerator200Response = await getModerator(tenantId, id);
  const moderatorName: string | undefined = response.moderator?.name;
  console.log(moderatorName);
})();
[inline-code-end]
