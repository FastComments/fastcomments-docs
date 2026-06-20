## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| approved | boolean | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_comment_approved_response.py)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på post_set_comment_approval_status'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.set_comment_approved_response import SetCommentApprovedResponse
from client.rest import ApiException
from pprint import pprint

# Angivelse af host er valgfri og standarden er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Åbn en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    approved = True # bool |  (valgfri)
    sso = 'sso_example' # str |  (valgfri)

    try:
        api_response = api_instance.post_set_comment_approval_status(comment_id, approved=approved, sso=sso)
        print("The response of ModerationApi->post_set_comment_approval_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_set_comment_approval_status: %s\n" % e)
[inline-code-end]