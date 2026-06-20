---
Vorige commentatoren op de pagina die NIET momenteel online zijn. Gesorteerd op displayName.
Gebruik dit nadat /users/online is uitgeput om een "Leden" sectie weer te geven.
Cursor-paginering op commenterName: server doorloopt de gedeeltelijke {tenantId, urlId, commenterName}
index vanaf afterName voorwaarts via $gt, geen $skip-kosten.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Pagina-URL-identificatie (server-side opgeschoond). |
| afterName | string | query | No | Cursor: geef nextAfterName mee uit de vorige reactie. |
| afterUserId | string | query | No | Cursor-tiebreaker: geef nextAfterUserId mee uit de vorige reactie. Vereist wanneer afterName is ingesteld zodat naamgelijkheden geen items laten vallen. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Example

[inline-code-attrs-start title = 'get_offline_users Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Pagina-URL-identificatie (server-side opgeschoond).
opts = {
  after_name: 'after_name_example', # String | Cursor: geef nextAfterName mee uit de vorige reactie.
  after_user_id: 'after_user_id_example' # String | Cursor-tiebreaker: geef nextAfterUserId mee uit de vorige reactie. Vereist wanneer afterName is ingesteld zodat naamgelijkheden geen items laten vallen.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]

---