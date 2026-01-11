## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| isLive | boolean | query | Nie |  |
| doSpamCheck | boolean | query | Nie |  |
| sendEmails | boolean | query | Nie |  |
| populateNotifications | boolean | query | Nie |  |

## Odpowiedź

Zwraca: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/save_comment200_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład save_comments_bulk'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_comment_params import CreateCommentParams
from client.models.save_comment200_response import SaveComment200Response
from client.rest import ApiException
from pprint import pprint

# Określenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klient musi skonfigurować parametry uwierzytelniania i autoryzacji
# zgodnie z polityką bezpieczeństwa serwera API.
# Przykłady dla każdej metody uwierzytelniania podane są poniżej, użyj przykładu, który
# pasuje do Twojego przypadku użycia uwierzytelniania.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Otwórz kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_comment_params = [client.CreateCommentParams()] # List[CreateCommentParams] | 
    is_live = True # bool |  (opcjonalne)
    do_spam_check = True # bool |  (opcjonalne)
    send_emails = True # bool |  (opcjonalne)
    populate_notifications = True # bool |  (opcjonalne)

    try:
        api_response = api_instance.save_comments_bulk(tenant_id, create_comment_params, is_live=is_live, do_spam_check=do_spam_check, send_emails=send_emails, populate_notifications=populate_notifications)
        print("The response of DefaultApi->save_comments_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->save_comments_bulk: %s\n" % e)
[inline-code-end]