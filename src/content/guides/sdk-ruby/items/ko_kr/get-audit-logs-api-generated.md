## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| limit | number | query | 아니오 |  |
| skip | number | query | 아니오 |  |
| order | string | query | 아니오 |  |
| after | number | query | 아니오 |  |
| before | number | query | 아니오 |  |

## 응답

반환: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_audit_logs_response.rb)

## 예제

[inline-code-attrs-start title = 'get_audit_logs 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 인증 설정
FastCommentsClient.configure do |config|
  # API 키 인증 구성: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API 키에 접두사(예: 'Bearer')를 설정하려면 다음 줄의 주석 처리를 제거하세요 (기본값: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  limit: 1.2, # Float | 
  skip: 1.2, # Float | 
  order: FastCommentsClient::SORTDIR::ASC, # SORTDIR | 
  after: 1.2, # Float | 
  before: 1.2 # Float | 
}

begin
  
  result = api_instance.get_audit_logs(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_audit_logs: #{e}"
end
[inline-code-end]