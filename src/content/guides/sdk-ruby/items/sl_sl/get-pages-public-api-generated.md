Prikaže seznam strani za najemnika. Uporablja ga namizni odjemalec FChat za pripravo seznama sob.
Zahteva, da je `enableFChat` nastavljen na true v razrešeni prilagojeni konfiguraciji za vsako stran.
Strani, ki zahtevajo SSO, so filtrirane glede na skupinski dostop uporabnika, ki pošilja zahtevo.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Opak kazalec paginacije, vrnjen kot `nextCursor` iz predhodne zahteve. Povezan z istim `sortBy`. |
| limit | integer | query | No | 1..200, privzeto 50 |
| q | string | query | No | Neobvezno filtiranje po začetku naslova, brez razlikovanja med velikimi in malimi črkami. |
| sortBy | string | query | No | Vrstni red sortiranja. `updatedAt` (privzeto, najnovejše prve), `commentCount` (največ komentarjev prve), ali `title` (po abecedi). |
| hasComments | boolean | query | No | Če je true, vrni samo strani z vsaj enim komentarjem. |

## Response

Vrača: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Example

[inline-code-attrs-start title = 'Primer get_pages_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Opak kazalec paginacije, vrnjen kot `nextCursor` iz predhodne zahteve. Povezan z istim `sortBy`.
  limit: 56, # Integer | 1..200, privzeto 50
  q: 'q_example', # String | Neobvezno filtiranje po začetku naslova, brez razlikovanja med velikimi in malimi črkami.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Vrstni red sortiranja. `updatedAt` (privzeto, najnovejše prve), `commentCount` (največ komentarjev prve), ali `title` (po abecedi).
  has_comments: true # Boolean | Če je true, vrni samo strani z vsaj enim komentarjem.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]