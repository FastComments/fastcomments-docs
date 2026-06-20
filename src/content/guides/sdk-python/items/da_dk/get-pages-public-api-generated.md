Lister sider for en tenant. Bruges af FChat desktop-klienten til at udfylde sin rumliste.
Kræver, at `enableFChat` er sand i den endelige brugerdefinerede konfiguration for hver side.
Sider, der kræver SSO, filtreres i forhold til den anmodende brugers gruppeadgang.

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Uigennemsigtig pagineringscursor returneret som `nextCursor` fra en tidligere anmodning. Bundet til samme `sortBy`. |
| limit | integer | query | No | 1..200, standard 50 |
| q | string | query | No | Valgfrit case-insensitivt filter på titelpræfiks. |
| sortBy | string | query | No | Sorteringsrækkefølge. `updatedAt` (standard, nyeste først), `commentCount` (flest kommentarer først), eller `title` (alfabetisk). |
| hasComments | boolean | query | No | Hvis sand, returnér kun sider med mindst én kommentar. |

## Svar

Returnerer: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_pages_public Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at definere host, og standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Åbn en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Uigennemsigtig pagineringscursor returneret som `nextCursor` fra en tidligere anmodning. Bundet til samme `sortBy`. (valgfri)
    limit = 56 # int | 1..200, standard 50 (valgfri)
    q = 'q_example' # str | Valgfrit case-insensitivt filter på titelpræfiks. (valgfri)
    sort_by = client.PagesSortBy() # PagesSortBy | Sorteringsrækkefølge. `updatedAt` (standard, nyeste først), `commentCount` (flest kommentarer først), eller `title` (alfabetisk). (valgfri)
    has_comments = True # bool | Hvis sand, returnér kun sider med mindst én kommentar. (valgfri)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]