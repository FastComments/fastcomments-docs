---
Tidligere kommentatorer på siden, som IKKE er online lige nu. Sorteret efter displayName.
Brug dette efter at have udtømt /users/online for at gengive en "Medlemmer"-sektion.
Cursor-paginering på commenterName: serveren gennemgår den delvise {tenantId, urlId, commenterName}
indekser fra afterName fremad via $gt, ingen $skip-omkostning.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Side-URL-identifikator (renset på serversiden). |
| afterName | string | query | No | Cursor: send nextAfterName fra det forrige svar. |
| afterUserId | string | query | No | Cursor tiebreaker: send nextAfterUserId fra det forrige svar. Påkrævet når afterName er angivet, så navne-ligheder ikke udelader poster. |

## Response

Returnerer: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Example

[inline-code-attrs-start title = 'Eksempel på get_offline_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Side-URL-identifikator (renset på serversiden).
opts = {
  after_name: 'after_name_example', # String | Cursor: send nextAfterName fra det forrige svar.
  after_user_id: 'after_user_id_example' # String | Cursor tiebreaker: send nextAfterUserId fra det forrige svar. Påkrævet når afterName er angivet, så navne-ligheder ikke udelader poster.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]

---