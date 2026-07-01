## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createAPISSOUserData | CreateAPISSOUserData | Nej |  |

## Svar

Returnerer: [`Option[AddSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_sso_user_api_response.nim)

## Eksempel

[inline-code-attrs-start title = 'addSSOUser Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.addSSOUser(tenantId = "my-tenant-123", createAPISSOUserData = CreateAPISSOUserData())
if optResp.isSome:
  let userResp = optResp.get()
[inline-code-end]