## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 아니오 |  |

## 응답

반환: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/add_hash_tag200_response.rb)

## 예제

[inline-code-attrs-start title = 'add_hash_tag 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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
opts = {
  tenant_id: 'tenant_id_example', # String | 
  create_hash_tag_body: FastCommentsClient::CreateHashTagBody.new({tag: 'tag_example'}) # CreateHashTagBody | 
}

begin
  
  result = api_instance.add_hash_tag(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->add_hash_tag: #{e}"
end
[inline-code-end]