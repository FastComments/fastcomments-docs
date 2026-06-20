Widzowie strony będący obecnie online: osoby, których sesja websocket jest teraz subskrybowana na tę stronę.
Zwraca anonCount + totalCount (subskrybenci w całym pokoju, w tym anonimowi widzowie, których nie wyliczamy).

## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| urlId | string | query | Tak | Identyfikator URL strony (oczyszczany po stronie serwera). |
| afterName | string | query | Nie | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. |
| afterUserId | string | query | Nie | Rozstrzygacz kursorów: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby przy remisach nazw nie utracić wpisów. |

## Odpowiedź

Zwraca: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Zdefiniowanie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Otwórz kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identyfikator URL strony (oczyszczany po stronie serwera).
    after_name = 'after_name_example' # str | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. (opcjonalne)
    after_user_id = 'after_user_id_example' # str | Rozstrzygacz kursorów: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby przy remisach nazw nie utracić wpisów. (opcjonalne)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]