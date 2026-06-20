---
## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| questionId | string | query | 아니요 |  |
| questionIds | array | query | 아니요 |  |
| urlId | string | query | 아니요 |  |
| startDate | string | query | 아니요 |  |
| forceRecalculate | boolean | query | 아니요 |  |
| minValue | number | query | 아니요 |  |
| maxValue | number | query | 아니요 |  |
| limit | number | query | 아니요 |  |

## 응답

반환: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/combine_question_results_with_comments_response.rb)

## 예제

[inline-code-attrs-start title = 'combine_comments_with_question_results 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 인증 설정
FastCommentsClient.configure do |config|
  # API 키 인증 구성: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API 키에 접두사를 설정하려면 다음 줄의 주석을 해제하세요. 예: 'Bearer' (기본값: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  question_id: 'question_id_example', # String | 
  question_ids: ['inner_example'], # Array<String> | 
  url_id: 'url_id_example', # String | 
  start_date: Time.parse('2013-10-20T19:20:30+01:00'), # Time | 
  force_recalculate: true, # Boolean | 
  min_value: 1.2, # Float | 
  max_value: 1.2, # Float | 
  limit: 1.2 # Float | 
}

begin
  
  result = api_instance.combine_comments_with_question_results(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->combine_comments_with_question_results: #{e}"
end
[inline-code-end]

---