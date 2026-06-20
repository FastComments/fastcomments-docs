---
## Parametri

| Nome | Tipo | Richiesto | Descrizione |
|------|------|----------|-------------|
| value | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`Option[ModerationPageSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_page_search_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di getSearchPages'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSearchPages(value = "news/politics/election-2024", sso = "sso-user-7f3b9c")
if response.isSome:
  let pageSearch = response.get()
  echo "Moderation page search returned"
else:
  echo "No moderation pages found"
[inline-code-end]

---