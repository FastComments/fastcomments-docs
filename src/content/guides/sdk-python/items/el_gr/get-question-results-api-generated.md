## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| urlId | string | query | Όχι |  |
| userId | string | query | Όχι |  |
| startDate | string | query | Όχι |  |
| questionId | string | query | Όχι |  |
| questionIds | string | query | Όχι |  |
| skip | number | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_results_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_results_response import GetQuestionResultsResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλεγμένο: https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους ρυθμίσεων.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο πελάτης πρέπει να ρυθμίσει τις παραμέτρους αυθεντικοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφαλείας του διακομιστή API.
# Παρακάτω παρέχονται παραδείγματα για κάθε μέθοδο αυθεντικοποίησης, χρησιμοποιήστε το παράδειγμα που
# ικανοποιεί την περίπτωση χρήσης αυθεντικοποίησής σας.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Αποσχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Χρησιμοποιήστε ένα context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str |  (προαιρετικό)
    user_id = 'user_id_example' # str |  (προαιρετικό)
    start_date = 'start_date_example' # str |  (προαιρετικό)
    question_id = 'question_id_example' # str |  (προαιρετικό)
    question_ids = 'question_ids_example' # str |  (προαιρετικό)
    skip = 3.4 # float |  (προαιρετικό)

    try:
        api_response = api_instance.get_question_results(tenant_id, url_id=url_id, user_id=user_id, start_date=start_date, question_id=question_id, question_ids=question_ids, skip=skip)
        print("The response of DefaultApi->get_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_results: %s\n" % e)
[inline-code-end]