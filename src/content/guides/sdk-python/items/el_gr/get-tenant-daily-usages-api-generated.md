## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-------------|
| tenantId | string | query | Yes |  |
| yearNumber | number | query | No |  |
| monthNumber | number | query | No |  |
| dayNumber | number | query | No |  |
| skip | number | query | No |  |

## Απόκριση

Επιστρέφει: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_daily_usages_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_tenant_daily_usages'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetTenantDailyUsagesOptions
from client.models.get_tenant_daily_usages_response import GetTenantDailyUsagesResponse
from client.rest import ApiException
from pprint import pprint

# Ο καθορισμός του host είναι προαιρετικός και προεπιλεγμένος σε https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους παραμετροποίησης.
# Ο πελάτης πρέπει να διαμορφώσει τις παραμέτρους αυθεντικοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφαλείας του διακομιστή API.
# Παρέχονται παραδείγματα για κάθε μέθοδο αυθεντικοποίησης παρακάτω, χρησιμοποιήστε το παράδειγμα που
# καλύπτει τη χρήση αυθεντικοποίησής σας.

# Διαμόρφωση εξουσιοδότησης με κλειδί API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Αποσχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, εάν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισαγωγή ενός πλαισίου με μια διεπαφή του πελάτη API
with client.ApiClient(configuration) as api_client:
    # Δημιουργία μιας διεπαφής της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    year_number = 3.4 # float |  (προαιρετικό)
    month_number = 3.4 # float |  (προαιρετικό)
    day_number = 3.4 # float |  (προαιρετικό)
    skip = 3.4 # float |  (προαιρετικό)

    try:
        api_response = api_instance.get_tenant_daily_usages(tenant_id, GetTenantDailyUsagesOptions(year_number=year_number, month_number=month_number, day_number=day_number, skip=skip))
        print("The response of DefaultApi->get_tenant_daily_usages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_daily_usages: %s\n" % e)
[inline-code-end]