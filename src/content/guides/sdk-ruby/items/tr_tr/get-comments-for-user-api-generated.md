## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| userId | string | query | Hayır |  |
| direction | string | query | Hayır |  |
| repliesToUserId | string | query | Hayır |  |
| page | number | query | Hayır |  |
| includei10n | boolean | query | Hayır |  |
| locale | string | query | Hayır |  |
| isCrawler | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comments_for_user_response.rb)

## Örnek

[inline-code-attrs-start title = 'get_comments_for_user Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
opts = {
  user_id: 'user_id_example', # String | 
  direction: FastCommentsClient::SortDirections::OF, # SortDirections | 
  replies_to_user_id: 'replies_to_user_id_example', # String | 
  page: 1.2, # Float | 
  includei10n: true, # Boolean | 
  locale: 'locale_example', # String | 
  is_crawler: true # Boolean | 
}

begin
  
  result = api_instance.get_comments_for_user(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_comments_for_user: #{e}"
end
[inline-code-end]