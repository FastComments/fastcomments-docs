## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Nie |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Nie |  |
| updateComments | bool | Nie |  |

## Odpowiedź

Zwraca: [`Option[PatchSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_sso_user_api_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład patchSSOUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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