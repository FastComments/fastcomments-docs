## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| id | string | Nie |  |
| title | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład createV2PageReact'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createV2PageReact(
  tenantId = "my-tenant-123",
  urlId = "news/2026/06/fastcomments-release",
  id = "",
  title = ""
)
if response.isSome:
  let react = response.get()
  echo "Created page react: ", $react
else:
  echo "No react returned, HTTP status: ", $httpResponse.statusCode
[inline-code-end]

---