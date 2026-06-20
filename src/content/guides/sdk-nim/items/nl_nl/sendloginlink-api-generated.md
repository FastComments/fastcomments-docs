## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nee |  |
| redirectURL | string | Nee |  |

## Antwoord

Retourneert: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'sendLoginLink Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.sendLoginLink(tenantId = "my-tenant-123", id = "user-456", redirectURL = "https://app.newsportal.com/welcome")
if response.isSome:
  let apiResp = response.get()
  echo "Login link sent successfully"
else:
  echo "Failed to send login link, HTTP status: ", $httpResponse.status
[inline-code-end]

---