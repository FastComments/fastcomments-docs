## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| value | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_page_search_response.rb)

## 예제

[inline-code-attrs-start title = 'get_search_pages 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  value: 'value_example', # 문자열 | 
  sso: 'sso_example' # 문자열 | 
}

begin
  
  result = api_instance.get_search_pages(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_search_pages: #{e}"
end
[inline-code-end]

---