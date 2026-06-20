Informacije o več uporabnikih za najemnika. Ob danih userIds vrne prikazne informacije iz User / SSOUser.
Uporablja se v pripomočku za komentarje za obogatitev uporabnikov, ki so se pravkar pojavili prek dogodka prisotnosti.
Brez konteksta strani: zasebnost se uveljavlja enotno (zasebni profili so skriti).

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | Vejicami ločeni userIds. |

## Odgovor

Vrača: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_info_response.rb)

## Primer

[inline-code-attrs-start title = 'Primer get_users_info'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
ids = 'ids_example' # String | Vejicami ločeni userIds.

begin
  
  result = api_instance.get_users_info(tenant_id, ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_users_info: #{e}"
end
[inline-code-end]