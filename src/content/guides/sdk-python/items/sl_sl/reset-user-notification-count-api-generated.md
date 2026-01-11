## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| sso | string | query | Ne |  |

## Odziv

Vrne: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/reset_user_notifications200_response.py)

## Primer

[inline-code-attrs-start title = 'Primer reset_user_notification_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.reset_user_notifications200_response import ResetUserNotifications200Response
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    sso = 'sso_example' # str |  (neobvezno)

    try:
        api_response = api_instance.reset_user_notification_count(tenant_id, sso=sso)
        print("The response of PublicApi->reset_user_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->reset_user_notification_count: %s\n" % e)
[inline-code-end]

---