## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| batchJobId | string | שאילתה | לא |  |
| sso | string | שאילתה | לא |  |

## תגובה

מחזיר: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_export_status_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_api_export_status'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  batch_job_id: 'batch_job_id_example', # מחרוזת | 
  sso: 'sso_example' # מחרוזת | 
}

begin
  
  result = api_instance.get_api_export_status(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_api_export_status: #{e}"
end
[inline-code-end]

---