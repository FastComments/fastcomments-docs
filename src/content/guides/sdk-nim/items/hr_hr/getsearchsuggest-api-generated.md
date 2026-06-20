---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| textSearch | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`Option[ModerationSuggestResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_suggest_response.nim)

## Primjer

[inline-code-attrs-start title = 'getSearchSuggest Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSearchSuggest(textSearch = "suspicious comment with spammy links", sso = "sso-user-789")
if response.isSome:
  let suggest = response.get()
  echo "ModerationSuggestResponse:"
  echo suggest
else:
  echo "No moderation suggestions returned. HTTP status: ", httpResponse.status
[inline-code-end]

---