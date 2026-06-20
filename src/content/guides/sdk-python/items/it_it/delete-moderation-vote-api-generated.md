---
## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| commentId | string | path | Sì |  |
| voteId | string | path | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_delete_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di delete_moderation_vote'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_delete_response import VoteDeleteResponse
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e ha come valore predefinito https://fastcomments.com
# Consultare configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrare in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    vote_id = 'vote_id_example' # str | 
    sso = 'sso_example' # str |  (opzionale)

    try:
        api_response = api_instance.delete_moderation_vote(comment_id, vote_id, sso=sso)
        print("The response of ModerationApi->delete_moderation_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->delete_moderation_vote: %s\n" % e)
[inline-code-end]

---