Lister sider for en tenant. Bruges af FChat desktop-klienten til at udfylde dens rumliste.
Kræver, at `enableFChat` er `true` i den resulterende tilpassede konfiguration for hver side.
Sider, der kræver SSO, filtreres i forhold til den anmodende brugers gruppeadgang.

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| cursor | string | query | Nej | Opak pagineringscursor returneret som `nextCursor` fra en tidligere anmodning. Knyttet til samme `sortBy`. |
| limit | integer | query | Nej | 1..200, standard 50 |
| q | string | query | Nej | Valgfrit case-insensitivt titelpræfiksfilter. |
| sortBy | string | query | Nej | Sorteringsrækkefølge. `updatedAt` (standard, nyeste først), `commentCount` (flest kommentarer først), eller `title` (alfabetisk). |
| hasComments | boolean | query | Nej | Hvis `true`, returner kun sider med mindst én kommentar. |

## Svar

Returnerer: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Eksempel

[inline-code-attrs-start title = 'get_pages_public Eksempel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Opak pagineringscursor returneret som `nextCursor` fra en tidligere anmodning. Knyttet til samme `sortBy`.
  limit: 56, # Integer | 1..200, standard 50
  q: 'q_example', # String | Valgfrit case-insensitivt titelpræfiksfilter.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Sorteringsrækkefølge. `updatedAt` (standard, nyeste først), `commentCount` (flest kommentarer først), eller `title` (alfabetisk).
  has_comments: true # Boolean | Hvis `true`, returner kun sider med mindst én kommentar.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]

---