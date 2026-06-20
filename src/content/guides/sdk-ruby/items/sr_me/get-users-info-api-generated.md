Skupne informacije o korisnicima za tenant. Za zadate userIds, vraća podatke za prikaz iz User / SSOUser.
Koristi widget za komentare da obogati korisnike koji su se upravo pojavili putem događaja prisutnosti.
Nema konteksta stranice: privatnost se primjenjuje dosljedno (privatni profili su maskirani).

## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| ids | string | query | Da | userIds odvojeni zarezima. |

## Odgovor

Vraća: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_info_response.rb)

## Primjer

[inline-code-attrs-start title = 'get_users_info Primjer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
ids = 'ids_example' # String | userIds odvojeni zarezima.

begin
  
  result = api_instance.get_users_info(tenant_id, ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_users_info: #{e}"
end
[inline-code-end]

---