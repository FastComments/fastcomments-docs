Skupne informacije o korisnicima za tenant. Za zadane userIds vraća informacije za prikaz iz User / SSOUser.
Koristi widget komentara za obogaćivanje korisnika koji su se upravo pojavili putem događaja prisutnosti.
Nema konteksta stranice: privatnost se primjenjuje jednako (privatni profili su maskirani).

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| ids | string | query | Da | userIds odvojeni zarezom. |

## Odgovor

Vraća: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_info_response.rb)

## Primjer

[inline-code-attrs-start title = 'Primjer get_users_info'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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