## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| options | GetSearchSuggestOptions | Nie |  |

## Odpowiedź

Zwraca: [`Option[ModerationSuggestResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_suggest_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getSearchSuggest'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (suggestOpt, httpResp) = client.getSearchSuggest(
  tenantId = "my-tenant-123",
  options = GetSearchSuggestOptions(),
)

if suggestOpt.isSome:
  let suggest = suggestOpt.get()
  echo suggest
[inline-code-end]