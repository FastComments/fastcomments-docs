## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | 경로 | 예 |  |
| locale | string | 쿼리 | 아니오 |  |
| rating | string | 쿼리 | 아니오 |  |
| page | number | 쿼리 | 아니오 |  |

## 응답

반환: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_gifs_trending_response.rb)

## 예제

[inline-code-attrs-start title = 'get_gifs_trending 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 문자열 | 
opts = {
  locale: 'locale_example', # 문자열 | 
  rating: 'rating_example', # 문자열 | 
  page: 1.2 # 실수 | 
}

begin
  
  result = api_instance.get_gifs_trending(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_gifs_trending: #{e}"
end
[inline-code-end]

---