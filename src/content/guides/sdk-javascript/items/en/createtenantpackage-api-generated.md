## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantPackageBody | CreateTenantPackageBody | Yes |  |

## Response

Returns: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackage200Response.ts)

## Example

[inline-code-attrs-start title = 'createTenantPackage Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_001';
const tosConfig: TOSConfig | undefined = undefined;
const createTenantPackageBody: CreateTenantPackageBody = {
  companyName: 'Acme Corporation',
  administratorEmail: 'it-admin@acme.com',
  enabled: true,
  defaultVoteStyle: 'updown' as unknown as VoteStyle,
  tosConfig
};
const result: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
[inline-code-end]
