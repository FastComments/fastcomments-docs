## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |

## Odgovor

Vrača: [`Option[DeletePageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_page_api_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer deletePage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deletePage(tenantId = "site-tenant-456", id = "news/winter-updates-2025")
if response.isSome:
  let deleted = response.get()
  echo "DeletePageAPIResponse:", deleted
else:
  echo "Delete failed, HTTP response:", httpResponse
[inline-code-end]

---