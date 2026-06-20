## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| broadcastId | string | query | Nee |  |
| isLive | boolean | query | Nee |  |
| doSpamCheck | boolean | query | Nee |  |
| skipDupCheck | boolean | query | Nee |  |

## Antwoord

Retourneert: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_feed_posts_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'create_feed_post Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_feed_post_params import CreateFeedPostParams
from client.models.create_feed_posts_response import CreateFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en standaard ingesteld op https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.
# De client moet de authenticatie- en autorisatieparameters
# configureren in overeenstemming met het beveiligingsbeleid van de API-server.
# Voor elke authenticatiemethode zijn hieronder voorbeelden opgenomen; gebruik het voorbeeld dat
# past bij uw authenticatiegeval.

# Configure API key authorization: api_key
# Configureer API key-autorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Verwijder hieronder de commentaarregel om een prefix in te stellen (bijv. Bearer) voor de API-sleutel, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_feed_post_params = client.CreateFeedPostParams() # CreateFeedPostParams | 
    broadcast_id = 'broadcast_id_example' # str |  (optioneel)
    is_live = True # bool |  (optioneel)
    do_spam_check = True # bool |  (optioneel)
    skip_dup_check = True # bool |  (optioneel)

    try:
        api_response = api_instance.create_feed_post(tenant_id, create_feed_post_params, broadcast_id=broadcast_id, is_live=is_live, do_spam_check=do_spam_check, skip_dup_check=skip_dup_check)
        print("The response of DefaultApi->create_feed_post:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_feed_post: %s\n" % e)
[inline-code-end]

---