## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | string | 아니오 |  |
| tenantId | string | 예 |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | 아니오 |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예제

[inline-code-attrs-start title = 'deleteHashTag 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteHashTag(
  tag = "",
  tenantId = "my-tenant-123",
  deleteHashTagRequestBody = DeleteHashTagRequestBody()
)

if response.isSome:
  let emptyResp = response.get()
  echo "Deleted hashtag for tenant my-tenant-123; response:", $emptyResp, " status:", $httpResponse.status
else:
  echo "No response body; status:", $httpResponse.status
[inline-code-end]

---