Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Връща anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).  
Текущо онлайн зрители на страница: хора, чиито уебсокет сесия е абонирана за страницата в момента.  
Връща anonCount + totalCount (абонати в цялата стая, включително анонимни зрители, които не изброяваме).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Page URL identifier (cleaned server-side). |
| afterName | string | query | No | Cursor: pass nextAfterName from the previous response. |
| afterUserId | string | query | No | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Example

[inline-code-attrs-start title = 'get_online_users Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOnlineUsersOptions
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Page URL identifier (cleaned server-side).
    after_name = 'after_name_example' # str | Cursor: pass nextAfterName from the previous response. (optional)
    after_user_id = 'after_user_id_example' # str | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. (optional)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, GetOnlineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]