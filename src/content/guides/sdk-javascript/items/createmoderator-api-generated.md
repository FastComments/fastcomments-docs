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
const tenantId: string = 'tenant_acme_corp_01';
const createModeratorBody: CreateModeratorBody = {
  email: 'jordan.lee+moderator@acme-corp.com',
  displayName: 'Jordan Lee',
  role: 'moderator',
  permissions: ['approve_comments', 'flag_for_review'],
  enabled: true,
  notes: 'Promoted from community volunteer' // optional parameter demonstrated
};
const newModerator: CreateModerator200Response = await createModerator(tenantId, createModeratorBody);
[inline-code-end]
