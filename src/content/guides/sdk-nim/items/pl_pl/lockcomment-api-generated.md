## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| broadcastId | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład lockComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.lockComment(tenantId = "news-tenant-42", commentId = "cmt-8f3a2b9d", broadcastId = "", sso = "")
if response.isSome:
  let apiResp = response.get()
  echo "Locked comment successfully for tenant news-tenant-42"
else:
  echo "Failed to lock comment, HTTP status: ", $httpResponse.status
[inline-code-end]

---