## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| domainToUpdate | string | path | 예 |  |

## 응답

반환: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/patch_domain_config_response.rb)

## 예제

[inline-code-attrs-start title = 'patch_domain_config 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 인증 설정
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API 키의 접두사를 설정하려면 다음 줄의 주석을 제거하세요. 예: 'Bearer' (기본값: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
domain_to_update = 'domain_to_update_example' # String | 
patch_domain_config_params = FastCommentsClient::PatchDomainConfigParams.new # PatchDomainConfigParams | 

begin
  
  result = api_instance.patch_domain_config(tenant_id, domain_to_update, patch_domain_config_params)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->patch_domain_config: #{e}"
end
[inline-code-end]

---