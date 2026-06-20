## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| spam | boolean | query | Nee |  |
| permNotSpam | boolean | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'post_set_comment_spam_status Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en staat standaard op https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    spam = True # bool |  (optional)
    perm_not_spam = True # bool |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.post_set_comment_spam_status(comment_id, spam=spam, perm_not_spam=perm_not_spam, sso=sso)
        print("The response of ModerationApi->post_set_comment_spam_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_set_comment_spam_status: %s\n" % e)
[inline-code-end]