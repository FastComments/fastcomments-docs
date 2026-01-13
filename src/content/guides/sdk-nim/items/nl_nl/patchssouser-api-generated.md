## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nee |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Nee |  |
| updateComments | bool | Nee |  |

## Respons

Retourneert: [`Option[PatchSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_sso_user_api_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'patchSSOUser Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchSSOUser(
  tenantId = "my-tenant-123",
  id = "user-789",
  updateAPISSOUserData = UpdateAPISSOUserData(
    externalId = "ext-987",
    username = "j.smith",
    email = "j.smith@news.example.com",
    displayName = "John Smith",
    roles = @["author", "editor"],
    avatarUrl = "https://cdn.news.example.com/avatars/j.smith.png"
  ),
  updateComments = true
)

if response.isSome:
  let patched = response.get()
  echo patched
[inline-code-end]