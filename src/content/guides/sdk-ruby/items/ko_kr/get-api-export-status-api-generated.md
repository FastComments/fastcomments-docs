## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| batchJobId | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_export_status_response.rb)

## 예제

[inline-code-attrs-start title = 'get_api_export_status 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  batch_job_id: 'batch_job_id_example', # 문자열 | 
  sso: 'sso_example' # 문자열 | 
}

begin
  
  result = api_instance.get_api_export_status(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_api_export_status: #{e}"
end
[inline-code-end]

---