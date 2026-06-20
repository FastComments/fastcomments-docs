---
Bulk-gebruikersinformatie voor een tenant. Gegeven userIds, retourneert weergave-informatie van User / SSOUser.
Wordt gebruikt door de commentaarwidget om gebruikers die zojuist zijn verschenen via een presence-event te verrijken.
Geen paginacontext: privacy wordt uniform gehandhaafd (privéprofielen worden gemaskeerd).

## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| ids | string | query | Ja | Door komma's gescheiden userIds. |

## Response

Retourneert: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_info_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'get_users_info Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
ids = 'ids_example' # String | Door komma's gescheiden userIds.

begin
  
  result = api_instance.get_users_info(tenant_id, ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_users_info: #{e}"
end
[inline-code-end]

---