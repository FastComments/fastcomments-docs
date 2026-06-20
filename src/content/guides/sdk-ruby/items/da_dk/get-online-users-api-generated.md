Nuværende online-seere af en side: personer hvis websocket-session er tilmeldt siden lige nu.
Returnerer anonCount + totalCount (abonnenter for hele rummet, inklusive anonyme seere, som vi ikke opremser).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Side-URL-identifikator (renset på serversiden). |
| afterName | string | query | No | Cursor: angiv nextAfterName fra det forrige svar. |
| afterUserId | string | query | No | Cursor tiebreaker: angiv nextAfterUserId fra det forrige svar. Påkrævet når afterName er angivet, så poster med samme navn ikke bliver udeladt. |

## Response

Returnerer: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## Example

[inline-code-attrs-start title = 'get_online_users Eksempel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Side-URL-identifikator (renset på serversiden).
opts = {
  after_name: 'after_name_example', # String | Cursor: angiv nextAfterName fra det forrige svar.
  after_user_id: 'after_user_id_example' # String | Cursor tiebreaker: angiv nextAfterUserId fra det forrige svar. Påkrævet når afterName er angivet, så poster med samme navn ikke bliver udeladt.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]