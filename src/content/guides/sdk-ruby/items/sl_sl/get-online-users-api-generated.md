Trenutno prisotni gledalci strani: osebe, katerih websocket seja je trenutno naročena na to stran.
Vrača anonCount + totalCount (naročniki v celotni sobi, vključno z anonimnimi gledalci, ki jih ne navajamo).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Page URL identifier (očisten na strežniški strani). |
| afterName | string | query | Ne | Kazalec: pošljite nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | Ne | Kazalec za razrešitev izenačitev: pošljite nextAfterUserId iz prejšnjega odgovora. Zahtevano, ko je afterName nastavljeno, da se vnosi ob izenačenju imen ne izgubijo. |

## Response

Vrača: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## Primer

[inline-code-attrs-start title = 'get_online_users Primer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Identifikator URL strani (očisten na strežniški strani).
opts = {
  after_name: 'after_name_example', # String | Kazalec: pošljite nextAfterName iz prejšnjega odgovora.
  after_user_id: 'after_user_id_example' # String | Kazalec za razrešitev izenačitev: pošljite nextAfterUserId iz prejšnjega odgovora. Zahtevano, ko je afterName nastavljeno, da se vnosi ob izenačenju imen ne izgubijo.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]