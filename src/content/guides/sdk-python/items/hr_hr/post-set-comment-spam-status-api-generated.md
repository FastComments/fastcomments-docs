## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | upit | Da |  |
| commentId | string | put | Da |  |
| spam | boolean | upit | Ne |  |
| permNotSpam | boolean | upit | Ne |  |
| broadcastId | string | upit | Ne |  |
| sso | string | upit | Ne |  |

## Odgovor

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Primjer

[inline-code-attrs-start title = 'post_set_comment_spam_status Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PostSetCommentSpamStatusOptions
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je neobavezno i zadano je https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Stvorite instancu API klase
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    spam = True # bool |  (neobavezno)
    perm_not_spam = True # bool |  (neobavezno)
    broadcast_id = 'broadcast_id_example' # str |  (neobavezno)
    sso = 'sso_example' # str |  (neobavezno)

    try:
        api_response = api_instance.post_set_comment_spam_status(tenant_id, comment_id, PostSetCommentSpamStatusOptions(spam=spam, perm_not_spam=perm_not_spam, broadcast_id=broadcast_id, sso=sso))
        print("The response of ModerationApi->post_set_comment_spam_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_set_comment_spam_status: %s\n" % e)
[inline-code-end]