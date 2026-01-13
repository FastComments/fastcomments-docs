## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |

## 응답

반환: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_email_template200_response.rb)

## 예제

[inline-code-attrs-start title = 'get_email_template 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 인증 설정
FastCommentsClient.configure do |config|
  # API 키 인증 구성: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API 키에 접두사(예: 'Bearer')를 설정하려면 다음 줄의 주석을 해제하세요 (기본값: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # 문자열 | 
id = 'id_example' # 문자열 | 

begin
  
  result = api_instance.get_email_template(tenant_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_email_template: #{e}"
end
[inline-code-end]