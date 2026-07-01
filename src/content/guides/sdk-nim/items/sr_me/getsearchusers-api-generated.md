## Parameters

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| options | GetSearchUsersOptions | Ne |  |

## Response

Vraća: [`Option[ModerationUserSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_user_search_response.nim)

## Example

[inline-code-attrs-start title = 'Primjer getSearchUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (searchRes, httpRes) = client.getSearchUsers(tenantId = "my-tenant-123", options = default(GetSearchUsersOptions))
if searchRes.isSome:
  let data = searchRes.get()
  echo data
[inline-code-end]