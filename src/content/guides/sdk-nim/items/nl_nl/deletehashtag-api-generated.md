---
## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tag | string | Nee |  |
| tenantId | string | Ja |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | Nee |  |

## Antwoord

Geeft terug: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'deleteHashTag Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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