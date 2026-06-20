## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| userId | string | query | No |  |
| badgeId | string | query | No |  |
| type | number | query | No |  |
| displayedOnComments | boolean | query | No |  |
| limit | number | query | No |  |
| skip | number | query | No |  |

## Risposta

Restituisce: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badges_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_user_badges'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badges_response import APIGetUserBadgesResponse
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è facoltativa e predefinita a https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la politica di sicurezza del server API.
# Esempi per ogni metodo di autenticazione sono forniti di seguito, usa l'esempio che
# soddisfa il tuo caso d'uso di autenticazione.
# Configura l'autorizzazione tramite API key: api_key
# Rimuovi il commento qui sotto per impostare il prefisso (es. Bearer) per API key, se necessario
# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (opzionale)
    badge_id = 'badge_id_example' # str |  (opzionale)
    type = 3.4 # float |  (opzionale)
    displayed_on_comments = True # bool |  (opzionale)
    limit = 3.4 # float |  (opzionale)
    skip = 3.4 # float |  (opzionale)

    try:
        api_response = api_instance.get_user_badges(tenant_id, user_id=user_id, badge_id=badge_id, type=type, displayed_on_comments=displayed_on_comments, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badges:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badges: %s\n" % e)
[inline-code-end]