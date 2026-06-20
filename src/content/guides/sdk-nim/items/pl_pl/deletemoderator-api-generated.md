## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Nie |  |
| sendEmail | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład deleteModerator'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteModerator(tenantId = "my-tenant-123", id = "", sendEmail = "")
if response.isSome:
  let apiEmpty = response.get()
  echo "Moderator deleted successfully for tenant my-tenant-123"
else:
  echo "No response returned; inspect httpResponse"
[inline-code-end]

---