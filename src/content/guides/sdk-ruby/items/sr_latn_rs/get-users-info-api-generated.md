---
Grupne informacije o korisnicima za tenant. Datim userIds, vraća prikazne informacije iz User / SSOUser.
Koristi se u komentarskom vidžetu da obogati korisnike koji su se upravo pojavili putem događaja prisustva.
Nema konteksta stranice: privatnost se primenjuje jednako (privatni profili su maskirani).

## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| ids | string | query | Da | userIds odvojeni zarezom. |

## Odgovor

Vraća: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_info_response.rb)

## Primer

[inline-code-attrs-start title = 'get_users_info Primer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
ids = 'ids_example' # String | userIds odvojeni zarezom.

begin
  
  result = api_instance.get_users_info(tenant_id, ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_users_info: #{e}"
end
[inline-code-end]

---