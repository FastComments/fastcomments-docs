## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |

## 응답

반환: [`Option[GetModeratorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderator_response.nim)

## 예제

[inline-code-attrs-start title = 'getModerator 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerator(tenantId = "my-tenant-123", id = "mod-456")
if response.isSome:
  let moderator = response.get()
  echo moderator
else:
  echo "Moderator not found, HTTP status: ", $httpResponse.status
[inline-code-end]

---