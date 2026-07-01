## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| options | GetSearchUsersOptions | Nee |  |

## Response

Retourneert: [`Option[ModerationUserSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_user_search_response.nim)

## Example

[inline-code-attrs-start title = 'getSearchUsers Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (searchRes, httpRes) = client.getSearchUsers(tenantId = "my-tenant-123", options = default(GetSearchUsersOptions))
if searchRes.isSome:
  let data = searchRes.get()
  echo data
[inline-code-end]