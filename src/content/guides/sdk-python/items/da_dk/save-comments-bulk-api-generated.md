## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| isLive | boolean | query | Nej |  |
| doSpamCheck | boolean | query | Nej |  |
| sendEmails | boolean | query | Nej |  |
| populateNotifications | boolean | query | Nej |  |

## Respons

Returnerer: [`SaveCommentsBulkResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/save_comments_bulk_response.py)

## Eksempel

[inline-code-attrs-start title = 'save_comments_bulk Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_comment_params import CreateCommentParams
from client.models.save_comments_bulk_response import SaveCommentsBulkResponse
from client.rest import ApiException
from pprint import pprint

# Angivelse af host er valgfri og standardværdien er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere godkendelses- og autorisationsparametrene
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver godkendelsesmetode er vist nedenfor — brug det eksempel, der
# passer til dit auth-brugstilfælde.

# Konfigurer API-nøgleautorisation: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommentaren nedenfor for at indstille præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_comment_params = [client.CreateCommentParams()] # List[CreateCommentParams] | 
    is_live = True # bool |  (valgfri)
    do_spam_check = True # bool |  (valgfri)
    send_emails = True # bool |  (valgfri)
    populate_notifications = True # bool |  (valgfri)

    try:
        api_response = api_instance.save_comments_bulk(tenant_id, create_comment_params, is_live=is_live, do_spam_check=do_spam_check, send_emails=send_emails, populate_notifications=populate_notifications)
        # Svaret fra DefaultApi->save_comments_bulk:\n
        pprint(api_response)
    except Exception as e:
        # Undtagelse ved kald til DefaultApi->save_comments_bulk: %s\n
        print("Exception when calling DefaultApi->save_comments_bulk: %s\n" % e)
[inline-code-end]