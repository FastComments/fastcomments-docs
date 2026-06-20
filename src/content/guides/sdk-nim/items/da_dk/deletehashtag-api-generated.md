## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tag | string | Nej |  |
| tenantId | string | Ja |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | Nej |  |

## Svar

Returnerer: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Eksempel

[inline-code-attrs-start title = 'deleteHashTag Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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