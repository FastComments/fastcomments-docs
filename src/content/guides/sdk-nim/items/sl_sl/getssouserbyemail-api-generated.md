## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| email | string | Ne |  |

## Odgovor

Vrne: [`Option[GetSSOUserByEmailAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_user_by_email_api_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getSSOUserByEmail'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSSOUserByEmail(tenantId = "my-tenant-123", email = "alice@newsco.com")
if response.isSome:
  let ssoUser = response.get()
  echo "SSO user found: ", ssoUser.email
else:
  echo "No SSO user found. HTTP status: ", httpResponse.status
[inline-code-end]

---