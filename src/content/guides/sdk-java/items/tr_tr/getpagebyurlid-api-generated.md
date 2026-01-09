## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |

## Yanıt

Döndürür: [`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPageByURLIdAPIResponse.java)

## Örnek

[inline-code-attrs-start title = 'getPageByURLId Örneği'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sınıfları içe aktar:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.auth.*;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");
    
    // API anahtarı yetkilendirmesini yapılandır: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // API anahtarına bir önek ayarlamak için aşağıdaki satırın yorumunu kaldırın, ör. "Token" (varsayılan null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String urlId = "urlId_example"; // String | 
    try {
      GetPageByURLIdAPIResponse result = apiInstance.getPageByURLId(tenantId, urlId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      // DefaultApi#getPageByURLId çağrılırken istisna oluştu
      System.err.println("Exception when calling DefaultApi#getPageByURLId");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]