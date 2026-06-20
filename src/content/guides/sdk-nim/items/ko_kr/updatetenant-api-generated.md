## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |
| updateTenantBody | UpdateTenantBody | 아니오 |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예제

[inline-code-attrs-start title = 'updateTenant 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateTenant(
  tenantId = "my-tenant-123",
  id = "settings",
  updateTenantBody = UpdateTenantBody(
    name = "My Tenant 123",
    enableModeration = true,
    allowedDomains = @["news.example.com", "blog.example.org"],
    maxCommentLength = 1000
  )
)

if response.isSome:
  let apiResp = response.get()
  echo "Tenant updated successfully: ", apiResp
else:
  echo "Failed to update tenant, HTTP status: ", httpResponse.status
[inline-code-end]

---