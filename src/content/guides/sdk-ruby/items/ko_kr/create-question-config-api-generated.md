## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |

## 응답

반환: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_question_config200_response.rb)

## 예제

[inline-code-attrs-start title = 'create_question_config 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 인증 설정
FastCommentsClient.configure do |config|
  # API 키 인증 구성: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API 키의 접두사를 설정하려면 다음 줄의 주석을 제거하세요(예: 'Bearer', 기본값: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_question_config_body = FastCommentsClient::CreateQuestionConfigBody.new({name: 'name_example', question: 'question_example', type: 'type_example', reporting_order: 3.56}) # CreateQuestionConfigBody | 

begin
  
  result = api_instance.create_question_config(tenant_id, create_question_config_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_question_config: #{e}"
end
[inline-code-end]