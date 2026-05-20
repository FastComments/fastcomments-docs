## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createModeratorBody | CreateModeratorBody | Yes |  |

## Response

Returns: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModerator200Response.ts)

## Example

[inline-code-attrs-start title = 'createModerator Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-01';
const createModeratorBody: CreateModeratorBody = {
  username: 'janedoe',
  email: 'jane.doe@acme-corp.com',
  displayName: 'Jane Doe',
  permissions: ['moderate_comments', 'review_reports'],
  notes: 'Primary moderator for product support channels' // optional parameter demonstrated
};
const result: CreateModerator200Response = await createModerator(tenantId, createModeratorBody);
[inline-code-end]
