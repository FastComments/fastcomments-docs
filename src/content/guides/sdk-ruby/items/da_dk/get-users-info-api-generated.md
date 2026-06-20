Bulk-brugerinfo for en tenant. Givet userIds returneres visningsinfo fra User / SSOUser.
Bruges af kommentar-widgeten til at berige brugere, der netop er dukket op via en presence-hændelse.
Ingen sidesammenhæng: privatliv håndhæves ensartet (private profiler er maskeret).

## Parameters

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| ids | string | query | Ja | Komma-separerede userIds. |

## Response

Returnerer: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_info_response.rb)

## Eksempel

[inline-code-attrs-start title = 'get_users_info Eksempel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
ids = 'ids_example' # String | Komma-separerede userIds.

begin
  
  result = api_instance.get_users_info(tenant_id, ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_users_info: #{e}"
end
[inline-code-end]