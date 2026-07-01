## Parameters

| 이름 | 타입 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | Yes |  |
| tag | string | No |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | No |  |

## Response

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예시

[inline-code-attrs-start title = 'deleteHashTag 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.deleteHashTag(tenantId = "my-tenant-123", tag = "sports", deleteHashTagRequestBody = DeleteHashTagRequestBody())
if apiResp.isSome:
  let emptyResp = apiResp.get()
[inline-code-end]