---
## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |

## Odpowiedź

Zwraca: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_tenant200_response.rb)

## Przykład

[inline-code-attrs-start title = 'create_tenant Przykład'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# konfiguracja autoryzacji
FastCommentsClient.configure do |config|
  # Konfiguracja autoryzacji kluczem API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Odkomentuj następującą linię, aby ustawić prefiks dla klucza API, np. 'Bearer' (domyślnie nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_tenant_body = FastCommentsClient::CreateTenantBody.new({name: 'name_example', domain_configuration: [FastCommentsClient::APIDomainConfiguration.new({id: 'id_example', domain: 'domain_example', created_at: Time.now})]}) # CreateTenantBody | 

begin
  
  result = api_instance.create_tenant(tenant_id, create_tenant_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_tenant: #{e}"
end
[inline-code-end]

---