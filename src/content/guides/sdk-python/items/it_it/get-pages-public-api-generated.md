Elenca le pagine per un tenant. Usato dal client desktop FChat per popolare la sua lista di stanze.
Richiede `enableFChat` to be true on the resolved custom config for each page.
Le pagine che richiedono SSO vengono filtrate in base all'accesso per gruppi dell'utente richiedente.

## Parametri

| Nome | Tipo | Location | Obbligatorio | Descrizione |
|------|------|----------|--------------|-------------|
| tenantId | string | path | Sì |  |
| cursor | string | query | No | Cursore opaco di paginazione restituito come `nextCursor` da una richiesta precedente. Collegato allo stesso `sortBy`. |
| limit | integer | query | No | 1..200, predefinito 50 |
| q | string | query | No | Filtro opzionale sul prefisso del titolo, insensibile alle maiuscole. |
| sortBy | string | query | No | Criterio di ordinamento. `updatedAt` (predefinito, i più recenti per primi), `commentCount` (più commenti per primi), oppure `title` (alfabetico). |
| hasComments | boolean | query | No | Se true, restituire solo pagine con almeno un commento. |

## Risposta

Restituisce: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Definire l'host è opzionale e per default è https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Cursore di paginazione opaco restituito come `nextCursor` da una richiesta precedente. Collegato allo stesso `sortBy`. (opzionale)
    limit = 56 # int | 1..200, predefinito 50 (opzionale)
    q = 'q_example' # str | Filtro opzionale sul prefisso del titolo, insensibile alle maiuscole. (opzionale)
    sort_by = client.PagesSortBy() # PagesSortBy | Criterio di ordinamento. `updatedAt` (predefinito, i più recenti per primi), `commentCount` (più commenti per primi), oppure `title` (alfabetico). (opzionale)
    has_comments = True # bool | Se true, restituire solo pagine con almeno un commento. (opzionale)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]