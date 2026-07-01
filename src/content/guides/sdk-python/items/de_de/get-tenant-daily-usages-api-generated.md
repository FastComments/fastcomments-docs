## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| yearNumber | number | query | Nein |  |
| monthNumber | number | query | Nein |  |
| dayNumber | number | query | Nein |  |
| skip | number | query | Nein |  |

## Antwort

Rückgabe: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_daily_usages_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_tenant_daily_usages Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetTenantDailyUsagesOptions
from client.models.get_tenant_daily_usages_response import GetTenantDailyUsagesResponse
from client.rest import ApiException
from pprint import pprint

# Das Definieren des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
# Der Client muss die Authentifizierungs‑ und Autorisierungsparameter konfigurieren
# gemäß der Sicherheitsrichtlinie des API‑Servers.
# Beispiele für jede Auth‑Methode sind unten angegeben, verwenden Sie das Beispiel, das
# Ihren Auth‑Anwendungsfall erfüllt.

# Konfigurieren der API‑Schlüssel‑Autorisation: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Betreten Sie einen Kontext mit einer Instanz des API‑Clients
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    year_number = 3.4 # float |  (optional)
    month_number = 3.4 # float |  (optional)
    day_number = 3.4 # float |  (optional)
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_tenant_daily_usages(tenant_id, GetTenantDailyUsagesOptions(year_number=year_number, month_number=month_number, day_number=day_number, skip=skip))
        print("The response of DefaultApi->get_tenant_daily_usages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_daily_usages: %s\n" % e)
[inline-code-end]