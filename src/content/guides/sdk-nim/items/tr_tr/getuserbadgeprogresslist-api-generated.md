## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| userId | string | Hayır |  |
| limit | float64 | Hayır |  |
| skip | float64 | Hayır |  |

## Yanıt

Döndürür: [`Option[APIGetUserBadgeProgressListResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badge_progress_list_response.nim)

## Örnek

[inline-code-attrs-start title = 'getUserBadgeProgressList Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBadgeProgressList(
  tenantId = "my-tenant-123",
  userId = "user-789",
  limit = 25.0,
  skip = 0.0
)

if response.isSome:
  let badgeProgress = response.get()
  echo "Received badge progress:", badgeProgress
else:
  echo "No badge progress; HTTP status: ", $httpResponse.status
[inline-code-end]

---