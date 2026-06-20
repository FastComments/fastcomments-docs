## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| text-search | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_suggest_response.rb)

## 예제

[inline-code-attrs-start title = 'get_search_suggest 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  text_search: 'text_search_example', # 문자열 | 
  sso: 'sso_example' # 문자열 | 
}

begin
  
  result = api_instance.get_search_suggest(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_search_suggest: #{e}"
end
[inline-code-end]

---