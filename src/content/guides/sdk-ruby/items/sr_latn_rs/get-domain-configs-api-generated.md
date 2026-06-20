## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vraća: [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_domain_configs_response.rb)

## Primer

[inline-code-attrs-start title = 'get_domain_configs Primer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# podešavanje autorizacije
FastCommentsClient.configure do |config|
  # Konfigurišite autorizaciju pomoću API ključa: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Otkomentarišite sledeću liniju da biste postavili prefiks za API ključ, npr. 'Bearer' (podrazumevano je nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 

begin
  
  result = api_instance.get_domain_configs(tenant_id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_domain_configs: #{e}"
end
[inline-code-end]