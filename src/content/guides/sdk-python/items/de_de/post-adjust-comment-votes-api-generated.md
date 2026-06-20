## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/adjust_votes_response.py)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für post_adjust_comment_votes'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.adjust_comment_votes_params import AdjustCommentVotesParams
from client.models.adjust_votes_response import AdjustVotesResponse
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Einen Kontext mit einer Instanz des API-Clients betreten
with client.ApiClient(configuration) as api_client:
    # Erstellen einer Instanz der API-Klasse
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    adjust_comment_votes_params = client.AdjustCommentVotesParams() # AdjustCommentVotesParams | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.post_adjust_comment_votes(comment_id, adjust_comment_votes_params, sso=sso)
        print("The response of ModerationApi->post_adjust_comment_votes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_adjust_comment_votes: %s\n" % e)
[inline-code-end]

---