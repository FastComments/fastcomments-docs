## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| questionId | string | query | Hayır |  |
| questionIds | array | query | Hayır |  |
| urlId | string | query | Hayır |  |
| timeBucket | string | query | Hayır |  |
| startDate | string | query | Hayır |  |
| forceRecalculate | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_question_results_response.py)

## Örnek

[inline-code-attrs-start title = 'aggregate_question_results Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregate_question_results_response import AggregateQuestionResultsResponse
from client.models.aggregate_time_bucket import AggregateTimeBucket
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini
# API sunucusunun güvenlik politikasına uygun şekilde yapılandırmalıdır.
# Her kimlik doğrulama yöntemi için örnekler aşağıda verilmiştir, 
# kullandığınız kimlik doğrulama durumuna uygun olanı kullanın.
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisinin bir örneğiyle bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  (isteğe bağlı)
    question_ids = ['question_ids_example'] # List[str] |  (isteğe bağlı)
    url_id = 'url_id_example' # str |  (isteğe bağlı)
    time_bucket = client.AggregateTimeBucket() # AggregateTimeBucket |  (isteğe bağlı)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (isteğe bağlı)
    force_recalculate = True # bool |  (isteğe bağlı)

    try:
        api_response = api_instance.aggregate_question_results(tenant_id, question_id=question_id, question_ids=question_ids, url_id=url_id, time_bucket=time_bucket, start_date=start_date, force_recalculate=force_recalculate)
        print("The response of DefaultApi->aggregate_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate_question_results: %s\n" % e)
[inline-code-end]

---