Popis stranica za tenant. Koristi ga FChat desktop klijent za popunjavanje svoje liste soba.
Zahtijeva da je `enableFChat` postavljeno na true u razriješenoj prilagođenoj konfiguraciji za svaku stranicu.
Stranice koje zahtijevaju SSO filtriraju se prema grupnom pristupu korisnika koji šalje zahtjev.

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozirni pokazivač paginacije vraćen kao `nextCursor` iz prethodnog zahtjeva. Povezan s istim `sortBy`. |
| limit | integer | query | Ne | 1..200, zadano 50 |
| q | string | query | Ne | Opcionalni filter prefiksa naslova koji nije osjetljiv na velika/mala slova. |
| sortBy | string | query | Ne | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno). |
| hasComments | boolean | query | Ne | Ako je true, vraćaju se samo stranice s najmanje jednim komentarom. |

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
  cursor: 'cursor_example', # String | Neprozirni pokazivač paginacije vraćen kao `nextCursor` iz prethodnog zahtjeva. Povezan s istim `sortBy`.
  limit: 56, # Integer | 1..200, zadano 50
  q: 'q_example', # String | Opcionalni filter prefiksa naslova koji nije osjetljiv na velika/mala slova.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno).
  has_comments: true # Boolean | Ako je true, vraćaju se samo stranice s najmanje jednim komentarom.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]