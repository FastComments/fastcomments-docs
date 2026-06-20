---
## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| batchJobId | string | poizvedba | Ne |  |
| sso | string | poizvedba | Ne |  |

## Odgovor

Vrne: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_export_status_response.rb)

## Primer

[inline-code-attrs-start title = 'get_api_export_status Primer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  batch_job_id: 'batch_job_id_example', # Niz | 
  sso: 'sso_example' # Niz | 
}

begin
  
  result = api_instance.get_api_export_status(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_api_export_status: #{e}"
end
[inline-code-end]

---