Prikazuje stranice za tenant. Koristi ga FChat desktop klijent za popunjavanje svoje liste soba. Zahtijeva da je `enableFChat` postavljen na true u riješenoj prilagođenoj konfiguraciji za svaku stranicu. Stranice koje zahtijevaju SSO filtriraju se prema grupnom pristupu korisnika koji šalje zahtjev.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Nečitljiv kursor za paginaciju vraćen kao `nextCursor` iz prethodnog zahtjeva. Vezan za isti `sortBy`. |
| limit | integer | query | No | 1..200, podrazumijevano 50 |
| q | string | query | No | Opcionalni filter prefiksa naslova koji nije osjetljiv na velika/mala slova. |
| sortBy | string | query | No | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno). |
| hasComments | boolean | query | No | Ako je true, vraćaju se samo stranice sa najmanje jednim komentarom. |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Primjer

[inline-code-attrs-start title = 'get_pages_public Primjer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Nečitljiv kursor za paginaciju vraćen kao `nextCursor` iz prethodnog zahtjeva. Vezan za isti `sortBy`.
  limit: 56, # Integer | 1..200, podrazumijevano 50
  q: 'q_example', # String | Opcionalni filter prefiksa naslova koji nije osjetljiv na velika/mala slova.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno).
  has_comments: true # Boolean | Ako je true, vraćaju se samo stranice sa najmanje jednim komentarom.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]