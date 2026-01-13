## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |
| skip | number | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetEmailTemplateRenderErrors200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_template_render_errors200_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_email_template_render_errors'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_template_render_errors200_response import GetEmailTemplateRenderErrors200Response
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλεγμένα χρησιμοποιείται το https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους ρύθμισης.
# Ο πελάτης πρέπει να ρυθμίσει τις παραμέτρους αυθεντικοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφαλείας του διακομιστή API.
# Παρακάτω παρέχονται παραδείγματα για κάθε μέθοδο αυθεντικοποίησης. Χρησιμοποιήστε το παράδειγμα που
# καλύπτει την περίπτωση χρήσης σας για αυθεντικοποίηση.
# Ρυθμίστε αυθεντικοποίηση με API key: api_key
# Αφαιρέστε το σχόλιο παρακάτω για να ρυθμίσετε το πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε ένα context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    skip = 3.4 # float |  (προαιρετικό)

    try:
        api_response = api_instance.get_email_template_render_errors(tenant_id, id, skip=skip)
        print("The response of DefaultApi->get_email_template_render_errors:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_email_template_render_errors: %s\n" % e)
[inline-code-end]