## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantPackageBody | CreateTenantPackageBody | No |  |

## Response

Returns: [`Option[CreateTenantPackage_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_package200response.nim)

## Example

[inline-code-attrs-start title = 'createTenantPackage Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createTenantPackage(
  tenantId = "my-tenant-123",
  createTenantPackageBody = CreateTenantPackageBody(
    packageName = "Pro Plan",
    priceUsdCents = 499,
    durationMonths = 12,
    features = @["priority-support", "custom-branding"],
    active = true,
    description = "Pro plan for news publishers"
  )
)
if response.isSome:
  let pkg = response.get()
  echo "Created package: ", pkg.packageName, " (id: ", pkg.id, ")"
[inline-code-end]
