Lijst pagina's voor een tenant. Gebruikt door de FChat desktop client om zijn kamerlijst te vullen. Vereist dat `enableFChat` op true staat in de opgeloste aangepaste configuratie voor elke pagina. Pagina's die SSO vereisen worden gefilterd op basis van de groepstoegang van de aanvragende gebruiker.

## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| cursor | string | query | Nee | Opaque pagineringscursor die als `nextCursor` wordt geretourneerd van een eerdere aanvraag. Gebonden aan dezelfde `sortBy`. |
| limit | integer | query | Nee | 1..200, standaard 50 |
| q | string | query | Nee | Optionele niet-hoofdlettergevoelige titel-prefixfilter. |
| sortBy | string | query | Nee | Sorteervolgorde. `updatedAt` (standaard, nieuwste eerst), `commentCount` (meeste opmerkingen eerst), of `title` (alfabetisch). |
| hasComments | boolean | query | Nee | Als true, alleen pagina's retourneren met ten minste één opmerking. |

## Response

Retourneert: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'get_pages_public Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Opaque pagineringscursor die als `nextCursor` wordt geretourneerd van een eerdere aanvraag. Gebonden aan dezelfde `sortBy`.
  limit: 56, # Integer | 1..200, standaard 50
  q: 'q_example', # String | Optionele niet-hoofdlettergevoelige titel-prefixfilter.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Sorteervolgorde. `updatedAt` (standaard, nieuwste eerst), `commentCount` (meeste opmerkingen eerst), of `title` (alfabetisch).
  has_comments: true # Boolean | Als true, alleen pagina's retourneren met ten minste één opmerking.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]