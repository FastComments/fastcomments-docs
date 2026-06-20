## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Пример

[inline-code-attrs-start title = 'replace_tenant_package Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# подешавање ауторизације
FastCommentsClient.configure do |config|
  # Конфигуриши API кључ за ауторизацију: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Откоментирај следећи ред да подесиш префикс за API кључ, нпр. 'Bearer' (подразумевано је nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
replace_tenant_package_body = FastCommentsClient::ReplaceTenantPackageBody.new({name: 'name_example', monthly_cost_usd: 3.56, yearly_cost_usd: 3.56, max_monthly_page_loads: 3.56, max_monthly_api_credits: 3.56, max_monthly_comments: 3.56, max_concurrent_users: 3.56, max_tenant_users: 3.56, max_sso_users: 3.56, max_moderators: 3.56, max_domains: 3.56, has_debranding: false, for_who_text: 'for_who_text_example', feature_taglines: ['feature_taglines_example'], has_flex_pricing: false}) # ReplaceTenantPackageBody | 

begin
  
  result = api_instance.replace_tenant_package(tenant_id, id, replace_tenant_package_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->replace_tenant_package: #{e}"
end
[inline-code-end]