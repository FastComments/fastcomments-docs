## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| contextUserId | string | query | Nee |  |
| doSpamCheck | boolean | query | Nee |  |
| isLive | boolean | query | Nee |  |

## Antwoord

Retourneert: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'update_comment Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.updatable_comment_params import UpdatableCommentParams
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en heeft als standaardwaarde https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters instellen
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voorbeelden voor elke auth-methode staan hieronder, gebruik het voorbeeld dat
# past bij uw authenticatiescenario.

# Configureer API-sleutelautorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal hieronder het commentaarteken weg om een prefix in te stellen (bijv. Bearer) voor de API-sleutel, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse aan
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    updatable_comment_params = client.UpdatableCommentParams() # UpdatableCommentParams | 
    context_user_id = 'context_user_id_example' # str |  (optioneel)
    do_spam_check = True # bool |  (optioneel)
    is_live = True # bool |  (optioneel)

    try:
        api_response = api_instance.update_comment(tenant_id, id, updatable_comment_params, context_user_id=context_user_id, do_spam_check=do_spam_check, is_live=is_live)
        print("The response of DefaultApi->update_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_comment: %s\n" % e)
[inline-code-end]