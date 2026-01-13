## Parametri

| Ime | Tip | Lokacija | Zahtevano | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| fromName | string | query | Da |  |

## Odgovor

Vrača: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/flag_comment_public200_response.rb)

## Primer

[inline-code-attrs-start title = 'Primer send_invite'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# nastavitev avtorizacije
FastCommentsClient.configure do |config|
  # Konfigurirajte avtorizacijo API ključa: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Odkomentirajte naslednjo vrstico za nastavitev predpone za API ključ, npr. 'Bearer' (privzeto nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
from_name = 'from_name_example' # String | 

begin
  
  result = api_instance.send_invite(tenant_id, id, from_name)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->send_invite: #{e}"
end
[inline-code-end]

---