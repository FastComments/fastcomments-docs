List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|-----|-----|--------------|--------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Undurchsichtiger Paginierungs-Cursor, zurückgegeben als `nextCursor` von einer vorherigen Anfrage. Gebunden an dasselbe `sortBy`. |
| limit | integer | query | No | 1..200, Standard 50 |
| q | string | query | No | Optionaler case‑insensitiver Titel‑Präfix‑Filter. |
| sortBy | string | query | No | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst) oder `title` (alphabetisch). |
| hasComments | boolean | query | No | Wenn true, nur Seiten zurückgeben, die mindestens einen Kommentar haben. |

## Response

Rückgabe: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Example

[inline-code-attrs-start title = 'get_pages_public Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Das Definieren des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Erstelle eine Instanz der API-Klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Undurchsichtiger Paginierungs-Cursor, zurückgegeben als `nextCursor` von einer vorherigen Anfrage. Gebunden an dasselbe `sortBy`. (optional)
    limit = 56 # int | 1..200, Standard 50 (optional)
    q = 'q_example' # str | Optionaler case‑insensitiver Titel‑Präfix‑Filter. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst) oder `title` (alphabetisch). (optional)
    has_comments = True # bool | Wenn true, nur Seiten zurückgeben, die mindestens einen Kommentar haben. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]

---