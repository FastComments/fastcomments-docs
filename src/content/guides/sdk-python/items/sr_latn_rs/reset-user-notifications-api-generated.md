## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| afterId | string | query | Ne |  |
| afterCreatedAt | integer | query | Ne |  |
| unreadOnly | boolean | query | Ne |  |
| dmOnly | boolean | query | Ne |  |
| noDm | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/reset_user_notifications_response.py)

## Primer

[inline-code-attrs-start title = 'Primer za reset_user_notifications'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.reset_user_notifications_response import ResetUserNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (neobavezno)
    after_created_at = 56 # int |  (neobavezno)
    unread_only = True # bool |  (neobavezno)
    dm_only = True # bool |  (neobavezno)
    no_dm = True # bool |  (neobavezno)
    sso = 'sso_example' # str |  (neobavezno)

    try:
        api_response = api_instance.reset_user_notifications(tenant_id, after_id=after_id, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, sso=sso)
        print("The response of PublicApi->reset_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->reset_user_notifications: %s\n" % e)
[inline-code-end]