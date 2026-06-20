## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| yearNumber | number | query | Nej |  |
| monthNumber | number | query | Nej |  |
| dayNumber | number | query | Nej |  |
| skip | number | query | Nej |  |

## Svar

Returnerer: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_daily_usages_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_tenant_daily_usages Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_daily_usages_response import GetTenantDailyUsagesResponse
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at definere host, og standarden er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere autentificerings- og autorisationsparametrene
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver auth-metode er givet nedenfor; brug det eksempel der
# opfylder dit auth-brugstilfælde.

# Konfigurer API-nøgleautorisation: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommentar for neden for at sætte præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    year_number = 3.4 # float |  (valgfri)
    month_number = 3.4 # float |  (valgfri)
    day_number = 3.4 # float |  (valgfri)
    skip = 3.4 # float |  (valgfri)

    try:
        api_response = api_instance.get_tenant_daily_usages(tenant_id, year_number=year_number, month_number=month_number, day_number=day_number, skip=skip)
        print("The response of DefaultApi->get_tenant_daily_usages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_daily_usages: %s\n" % e)
[inline-code-end]