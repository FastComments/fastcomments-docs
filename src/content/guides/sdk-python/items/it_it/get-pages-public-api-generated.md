Elenca le pagine per un tenant. Utilizzato dal client desktop FChat per popolare la sua lista di stanze.
Richiede che `enableFChat` sia true nella configurazione personalizzata risolta per ogni pagina.
Le pagine che richiedono SSO vengono filtrate in base all'accesso ai gruppi dell'utente richiedente.

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| cursor | string | query | No | Cursore di paginazione opaco restituito come `nextCursor` da una richiesta precedente. Legato allo stesso `sortBy`. |
| limit | integer | query | No | 1..200, predefinito 50 |
| q | string | query | No | Filtro opzionale per prefisso del titolo, senza distinzione maiuscole/minuscole. |
| sortBy | string | query | No | Ordinamento. `updatedAt` (predefinito, dal più recente), `commentCount` (dal maggior numero di commenti), o `title` (alfabetico). |
| hasComments | boolean | query | No | Se true, restituisce solo le pagine con almeno un commento. |

## Risposta

Restituisce: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Cursore di paginazione opaco restituito come `nextCursor` da una richiesta precedente. Legato allo stesso `sortBy`. (optional)
    limit = 56 # int | 1..200, predefinito 50 (optional)
    q = 'q_example' # str | Filtro opzionale per prefisso del titolo, senza distinzione maiuscole/minuscole. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Ordinamento. `updatedAt` (predefinito, dal più recente), `commentCount` (dal maggior numero di commenti), o `title` (alfabetico). (optional)
    has_comments = True # bool | Se true, restituisce solo le pagine con almeno un commento. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]