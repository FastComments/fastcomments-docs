## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| textSearch | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[ModerationSuggestResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_suggest_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getSearchSuggest'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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