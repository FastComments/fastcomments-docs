Bulk brugerinfo for en lejer. Givet bruger‑id'er, returnér display‑info fra User / SSOUser.  
Brugt af kommentarfunktionen til at berige brugere, som lige er dukket op via en tilstedeværelseshændelse.  
Ingen sidekontekst: privatliv håndhæves ensartet (private profiler maskeres).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | No |  |

## Response

Returns: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Example

[inline-code-attrs-start title = 'Eksempel på getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (usersInfoOpt, httpResp) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user42")
if usersInfoOpt.isSome:
  let usersInfo = usersInfoOpt.get()
[inline-code-end]