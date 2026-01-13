## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| pageSize | integer | query | Nej |  |
| afterId | string | query | Nej |  |
| includeContext | boolean | query | Nej |  |
| afterCreatedAt | integer | query | Nej |  |
| unreadOnly | boolean | query | Nej |  |
| dmOnly | boolean | query | Nej |  |
| noDm | boolean | query | Nej |  |
| includeTranslations | boolean | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_notifications200_response.py)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på get_user_notifications'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_notifications200_response import GetUserNotifications200Response
from client.rest import ApiException
from pprint import pprint

# Angivelse af host er valgfri og standarden er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page_size = 56 # int |  (valgfri)
    after_id = 'after_id_example' # str |  (valgfri)
    include_context = True # bool |  (valgfri)
    after_created_at = 56 # int |  (valgfri)
    unread_only = True # bool |  (valgfri)
    dm_only = True # bool |  (valgfri)
    no_dm = True # bool |  (valgfri)
    include_translations = True # bool |  (valgfri)
    sso = 'sso_example' # str |  (valgfri)

    try:
        api_response = api_instance.get_user_notifications(tenant_id, page_size=page_size, after_id=after_id, include_context=include_context, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, include_translations=include_translations, sso=sso)
        print("The response of PublicApi->get_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_notifications: %s\n" % e)
[inline-code-end]