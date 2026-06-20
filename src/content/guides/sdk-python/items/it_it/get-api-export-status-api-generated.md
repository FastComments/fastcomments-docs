## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| batchJobId | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_export_status_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_api_export_status'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_export_status_response import ModerationExportStatusResponse
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è facoltativa e il valore predefinito è https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.ModerationApi(api_client)
    batch_job_id = 'batch_job_id_example' # str |  (opzionale)
    sso = 'sso_example' # str |  (opzionale)

    try:
        api_response = api_instance.get_api_export_status(batch_job_id=batch_job_id, sso=sso)
        print("The response of ModerationApi->get_api_export_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_export_status: %s\n" % e)
[inline-code-end]