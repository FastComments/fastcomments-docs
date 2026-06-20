## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| urlId | string | Ja |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Beispiel

[inline-code-attrs-start title = 'putReopenThread Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.putReopenThread(urlId = "news/2026-election-analysis", sso = "")
if response.isSome:
  let apiResp = response.get()
  echo "Reopen succeeded, response: ", apiResp
else:
  echo "Reopen failed, HTTP status: ", httpResponse.status
[inline-code-end]

---