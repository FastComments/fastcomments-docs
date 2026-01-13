## Parametreler

| Ad | Type | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| questionId | string | query | Hayır |  |
| questionIds | array | query | Hayır |  |
| urlId | string | query | Hayır |  |
| startDate | string | query | Hayır |  |
| forceRecalculate | boolean | query | Hayır |  |
| minValue | number | query | Hayır |  |
| maxValue | number | query | Hayır |  |
| limit | number | query | Hayır |  |

## Yanıt

Döndürür: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/combine_comments_with_question_results200_response.py)

## Örnek

[inline-code-attrs-start title = 'combine_comments_with_question_results Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.combine_comments_with_question_results200_response import CombineCommentsWithQuestionResults200Response
from client.rest import ApiException
from pprint import pprint

# Sunucuyu tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Desteklenen tüm yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini
# API sunucusunun güvenlik politikasına uygun şekilde yapılandırmalıdır.
# Her kimlik doğrulama yöntemi için örnekler aşağıda sağlanmıştır, kimlik doğrulama kullanım durumunuza uyan örneği kullanın.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (ör. Bearer) ayarlamak için aşağıdaki yorumu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneği ile bir bağlam (context) girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  (isteğe bağlı)
    question_ids = ['question_ids_example'] # List[str] |  (isteğe bağlı)
    url_id = 'url_id_example' # str |  (isteğe bağlı)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (isteğe bağlı)
    force_recalculate = True # bool |  (isteğe bağlı)
    min_value = 3.4 # float |  (isteğe bağlı)
    max_value = 3.4 # float |  (isteğe bağlı)
    limit = 3.4 # float |  (isteğe bağlı)

    try:
        api_response = api_instance.combine_comments_with_question_results(tenant_id, question_id=question_id, question_ids=question_ids, url_id=url_id, start_date=start_date, force_recalculate=force_recalculate, min_value=min_value, max_value=max_value, limit=limit)
        print("The response of DefaultApi->combine_comments_with_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->combine_comments_with_question_results: %s\n" % e)
[inline-code-end]