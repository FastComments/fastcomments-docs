## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| urlId | string | Sì |  |
| sso | string | No |  |

## Risposta

Restituisce: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di putCloseThread'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.putCloseThread(urlId = "news/article-title", sso = "")
if response.isSome:
  let apiResp = response.get()
  echo "Thread closed successfully:", $apiResp
else:
  echo "Failed to close thread, HTTP response:", $httpResponse
[inline-code-end]

---