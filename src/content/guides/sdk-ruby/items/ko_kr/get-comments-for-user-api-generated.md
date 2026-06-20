## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| userId | string | query | 아니오 |  |
| direction | string | query | 아니오 |  |
| repliesToUserId | string | query | 아니오 |  |
| page | number | query | 아니오 |  |
| includei10n | boolean | query | 아니오 |  |
| locale | string | query | 아니오 |  |
| isCrawler | boolean | query | 아니오 |  |

## 응답

반환: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comments_for_user_response.rb)

## 예제

[inline-code-attrs-start title = 'get_comments_for_user 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
opts = {
  user_id: 'user_id_example', # 문자열(String) | 
  direction: FastCommentsClient::SortDirections::OF, # SortDirections(정렬 방향) | 
  replies_to_user_id: 'replies_to_user_id_example', # 문자열(String) | 
  page: 1.2, # 실수(Float) | 
  includei10n: true, # 불리언(Boolean) | 
  locale: 'locale_example', # 문자열(String) | 
  is_crawler: true # 불리언(Boolean) | 
}

begin
  
  result = api_instance.get_comments_for_user(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_comments_for_user: #{e}"
end
[inline-code-end]