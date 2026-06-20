## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Hayır |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Hayır |  |

## Yanıt

Döndürür: [`Option[APIEmptySuccessResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_success_response.nim)

## Örnek

[inline-code-attrs-start title = 'updateUserBadge Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateUserBadge(
  tenantId = "my-tenant-123",
  id = "user-456",
  updateUserBadgeParams = UpdateUserBadgeParams()
)

if response.isSome:
  let success = response.get()
  echo "Badge updated successfully"
else:
  echo "Badge update failed"
[inline-code-end]

---