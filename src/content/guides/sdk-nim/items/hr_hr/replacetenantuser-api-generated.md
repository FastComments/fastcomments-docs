## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| replaceTenantUserBody | ReplaceTenantUserBody | Ne |  |
| updateComments | string | Ne |  |

## Odgovor

Vraća: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer replaceTenantUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let replaceBody = ReplaceTenantUserBody(name: "Jane Doe", email: "jane.doe@example.com", roles: @["moderator", "contributor"], banned: false)
let (response, httpResponse) = client.replaceTenantUser(tenantId = "my-tenant-123", id = "user-789", replaceTenantUserBody = replaceBody, updateComments = "true")
if response.isSome:
  let updated = response.get()
  echo updated
else:
  echo "No response returned"
[inline-code-end]

---