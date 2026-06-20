---
Bulk brugeroplysninger for en tenant. Givet userIds, returnér visningsoplysninger fra User / SSOUser.
Bruges af comment widget til at berige brugere, der lige dukkede op via en presence event.
Ingen sidekontekst: privatliv håndhæves ensartet (private profiler er maskeret).

## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| ids | string | Nej |  |

## Svar

Returnerer: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getUsersInfo Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user-42,user-87")
if response.isSome:
  let usersInfo = response.get()
  echo "Retrieved users info:", usersInfo
else:
  echo "No users info returned. HTTP status:", httpResponse.status
[inline-code-end]

---