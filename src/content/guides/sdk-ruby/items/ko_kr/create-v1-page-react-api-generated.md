## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| title | string | query | 아니오 |  |

## 응답

반환: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_v1_page_react.rb)

## 예제

[inline-code-attrs-start title = 'create_v1_page_react 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 문자열 | 
url_id = 'url_id_example' # 문자열 | 
opts = {
  title: 'title_example' # 문자열 | 
}

begin
  
  result = api_instance.create_v1_page_react(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->create_v1_page_react: #{e}"
end
[inline-code-end]

---