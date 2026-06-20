## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlId | string | query | Ne | Koristi se za utvrđivanje da li je trenutna stranica pretplaćena. |
| pageSize | integer | query | Ne |  |
| afterId | string | query | Ne |  |
| includeContext | boolean | query | Ne |  |
| afterCreatedAt | integer | query | Ne |  |
| unreadOnly | boolean | query | Ne |  |
| dmOnly | boolean | query | Ne |  |
| noDm | boolean | query | Ne |  |
| includeTranslations | boolean | query | Ne |  |
| includeTenantNotifications | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_my_notifications_response.py)

## Primjer

[inline-code-attrs-start title = 'get_user_notifications Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_my_notifications_response import GetMyNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumijevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih konfiguracionih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Koristi se za utvrđivanje da li je trenutna stranica pretplaćena. (opcionalno)
    page_size = 56 # int |  (opcionalno)
    after_id = 'after_id_example' # str |  (opcionalno)
    include_context = True # bool |  (opcionalno)
    after_created_at = 56 # int |  (opcionalno)
    unread_only = True # bool |  (opcionalno)
    dm_only = True # bool |  (opcionalno)
    no_dm = True # bool |  (opcionalno)
    include_translations = True # bool |  (opcionalno)
    include_tenant_notifications = True # bool |  (opcionalno)
    sso = 'sso_example' # str |  (opcionalno)

    try:
        api_response = api_instance.get_user_notifications(tenant_id, url_id=url_id, page_size=page_size, after_id=after_id, include_context=include_context, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, include_translations=include_translations, include_tenant_notifications=include_tenant_notifications, sso=sso)
        print("The response of PublicApi->get_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_notifications: %s\n" % e)
[inline-code-end]