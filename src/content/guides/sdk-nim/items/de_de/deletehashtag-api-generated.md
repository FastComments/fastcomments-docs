## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tag | string | Nein |  |
| tenantId | string | Ja |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | Nein |  |

## Antwort

Gibt zurück: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Beispiel

[inline-code-attrs-start title = 'deleteHashTag Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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