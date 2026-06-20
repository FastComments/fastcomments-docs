Lista stranica za tenant. Koristi se od strane FChat desktop klijenta da popuni svoju listu soba. Zahteva da `enableFChat` bude true u rešenom prilagođenom podešavanju za svaku stranicu. Stranice koje zahtevaju SSO se filtriraju prema pristupu grupa korisnika koji šalje zahtev.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozirni kursor paginacije vraćen kao `nextCursor` iz prethodnog zahteva. Povezan sa istim `sortBy`. |
| limit | integer | query | Ne | 1..200, podrazumevano 50 |
| q | string | query | Ne | Opcioni filter prefiksa naslova koji nije osetljiv na velika/mala slova. |
| sortBy | string | query | Ne | Redosled sortiranja. `updatedAt` (podrazumevano, najnoviji prvi), `commentCount` (najviše komentara prvi), ili `title` (abecedno). |
| hasComments | boolean | query | Ne | Ako je true, vraćaju se samo stranice koje imaju najmanje jedan komentar. |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Primer

[inline-code-attrs-start title = 'Primer get_pages_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Neprozirni kursor paginacije vraćen kao `nextCursor` iz prethodnog zahteva. Povezan sa istim `sortBy`.
  limit: 56, # Integer | 1..200, podrazumevano 50
  q: 'q_example', # String | Opcioni filter prefiksa naslova koji nije osetljiv na velika/mala slova.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Redosled sortiranja. `updatedAt` (podrazumevano, najnoviji prvi), `commentCount` (najviše komentara prvi), ili `title` (abecedno).
  has_comments: true # Boolean | Ako je true, vraćaju se samo stranice koje imaju najmanje jedan komentar.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]