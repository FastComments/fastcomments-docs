Listet Seiten für einen Tenant auf. Wird vom FChat-Desktop-Client verwendet, um seine Raumliste zu füllen.
Erfordert, dass `enableFChat` in der aufgelösten benutzerdefinierten Konfiguration für jede Seite auf true gesetzt ist.
Seiten, die SSO erfordern, werden gegen die Gruppenzugriffsrechte des anfragenden Benutzers gefiltert.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| cursor | string | query | Nein | Opaker Paginierungs-Cursor, der als `nextCursor` von einer vorherigen Anfrage zurückgegeben wurde. An denselben `sortBy` gebunden. |
| limit | integer | query | Nein | 1..200, Standard 50 |
| q | string | query | Nein | Optionaler Filter für Titel-Präfix, Groß-/Kleinschreibung wird ignoriert. |
| sortBy | string | query | Nein | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst) oder `title` (alphabetisch). |
| hasComments | boolean | query | Nein | Wenn true, werden nur Seiten mit mindestens einem Kommentar zurückgegeben. |

## Antwort

Gibt zurück: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_pages_public Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
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
    cursor = 'cursor_example' # str | Opaker Paginierungs-Cursor, der als `nextCursor` von einer vorherigen Anfrage zurückgegeben wurde. An denselben `sortBy` gebunden. (optional)
    limit = 56 # int | 1..200, Standard 50 (optional)
    q = 'q_example' # str | Optionaler Filter für Titel-Präfix, Groß-/Kleinschreibung wird ignoriert. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst) oder `title` (alphabetisch). (optional)
    has_comments = True # bool | Wenn true, werden nur Seiten mit mindestens einem Kommentar zurückgegeben. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]