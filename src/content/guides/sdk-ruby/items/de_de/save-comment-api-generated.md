## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| isLive | boolean | query | Nein |  |
| doSpamCheck | boolean | query | Nein |  |
| sendEmails | boolean | query | Nein |  |
| populateNotifications | boolean | query | Nein |  |

## Antwort

Gibt zurück: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_save_comment_response.rb)

## Beispiel

[inline-code-attrs-start title = 'save_comment Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Autorisierung einrichten
FastCommentsClient.configure do |config|
  # API-Schlüssel-Autorisierung konfigurieren: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Kommentieren Sie die folgende Zeile aus, um ein Präfix für den API-Schlüssel festzulegen, z. B. 'Bearer' (Standard ist nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_comment_params = FastCommentsClient::CreateCommentParams.new({commenter_name: 'commenter_name_example', comment: 'comment_example', url: 'url_example', url_id: 'url_id_example', locale: 'locale_example'}) # CreateCommentParams | 
opts = {
  is_live: true, # Boolean | 
  do_spam_check: true, # Boolean | 
  send_emails: true, # Boolean | 
  populate_notifications: true # Boolean | 
}

begin
  
  result = api_instance.save_comment(tenant_id, create_comment_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->save_comment: #{e}"
end
[inline-code-end]

---