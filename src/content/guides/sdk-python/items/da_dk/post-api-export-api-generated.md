## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Nej |  |
| byIPFromComment | string | query | Nej |  |
| filters | string | query | Nej |  |
| searchFilters | string | query | Nej |  |
| sorts | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_export_response.py)

## Eksempel

[inline-code-attrs-start title = 'post_api_export Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_export_response import ModerationExportResponse
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at angive host; standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (optional)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (optional)
    filters = 'filters_example' # str |  (optional)
    search_filters = 'search_filters_example' # str |  (optional)
    sorts = 'sorts_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.post_api_export(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, sorts=sorts, sso=sso)
        print("The response of ModerationApi->post_api_export:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_api_export: %s\n" % e)
[inline-code-end]