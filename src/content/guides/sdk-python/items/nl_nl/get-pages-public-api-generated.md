List pagina's voor een tenant. Wordt gebruikt door de FChat desktopclient om de kamerlijst te vullen. Vereist dat `enableFChat` true is in de opgeloste aangepaste configuratie voor elke pagina. Pagina's die SSO vereisen worden gefilterd op basis van de groepsrechten van de aanvragende gebruiker.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Ondoorzichtige pagineringscursor geretourneerd als `nextCursor` van een eerdere aanvraag. Gebonden aan dezelfde `sortBy`. |
| limit | integer | query | No | 1..200, standaard 50 |
| q | string | query | No | Optionele hoofdletterongevoelige titel-voorvoegselfilter. |
| sortBy | string | query | No | Sorteervolgorde. `updatedAt` (standaard, nieuwste eerst), `commentCount` (meeste opmerkingen eerst), of `title` (alfabetisch). |
| hasComments | boolean | query | No | Indien true, alleen pagina's retourneren met ten minste één opmerking. |

## Respons

Retourneert: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_pages_public Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Ondoorzichtige pagineringscursor geretourneerd als `nextCursor` van een eerdere aanvraag. Gebonden aan dezelfde `sortBy`. (optioneel)
    limit = 56 # int | 1..200, standaard 50 (optioneel)
    q = 'q_example' # str | Optionele hoofdletterongevoelige titel-voorvoegselfilter. (optioneel)
    sort_by = client.PagesSortBy() # PagesSortBy | Sorteervolgorde. `updatedAt` (standaard, nieuwste eerst), `commentCount` (meeste opmerkingen eerst), of `title` (alfabetisch). (optioneel)
    has_comments = True # bool | Indien true, alleen pagina's retourneren met ten minste één opmerking. (optioneel)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]