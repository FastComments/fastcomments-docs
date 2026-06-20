---
## Parametri

| Ime | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Одговор

Враћа: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/add_domain_config_response.rb)

## Пример

[inline-code-attrs-start title = 'add_domain_config Primer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# постављање ауторизације
FastCommentsClient.configure do |config|
  # Konfigurišite autorizaciju API ključa: api_key
  # Одкоментаришите следећи ред да бисте поставили префикс за API кључ, нпр. 'Bearer' (задано: nil)
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
add_domain_config_params = FastCommentsClient::AddDomainConfigParams.new({domain: 'domain_example'}) # AddDomainConfigParams | 

begin
  
  result = api_instance.add_domain_config(tenant_id, add_domain_config_params)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->add_domain_config: #{e}"
end
[inline-code-end]

---