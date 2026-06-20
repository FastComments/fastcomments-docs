Elenca le pagine per un tenant. Utilizzato dal client desktop FChat per popolare la lista delle sue stanze. Richiede che `enableFChat` sia true nella configurazione personalizzata risolta per ogni pagina. Le pagine che richiedono SSO vengono filtrate in base ai gruppi di accesso dell'utente richiedente.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Cursore di paginazione opaco restituito come `nextCursor` da una richiesta precedente. Legato allo stesso `sortBy`. |
| limit | integer | query | No | 1..200, predefinito 50 |
| q | string | query | No | Filtro opzionale per prefisso del titolo non sensibile al maiuscolo/minuscolo. |
| sortBy | string | query | No | Ordine. `updatedAt` (predefinito, i più recenti per primi), `commentCount` (i più commentati per primi), o `title` (alfabetico). |
| hasComments | boolean | query | No | Se true, restituisce solo le pagine con almeno un commento. |

## Risposta

Restituisce: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio get_pages_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Cursore di paginazione opaco restituito come `nextCursor` da una richiesta precedente. Legato allo stesso `sortBy`.
  limit: 56, # Integer | 1..200, predefinito 50
  q: 'q_example', # String | Filtro opzionale per prefisso del titolo non sensibile a maiuscole/minuscole.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Ordine dei risultati. `updatedAt` (predefinito, i più recenti per primi), `commentCount` (i più commentati per primi), o `title` (alfabetico).
  has_comments: true # Boolean | Se true, restituisce solo le pagine con almeno un commento.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]