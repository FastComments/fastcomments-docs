## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| yearNumber | number | query | Hayır |  |
| monthNumber | number | query | Hayır |  |
| dayNumber | number | query | Hayır |  |
| skip | number | query | Hayır |  |

## Yanıt

Döndürür: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetTenantDailyUsages200Response.java)

## Örnek

[inline-code-attrs-start title = 'getTenantDailyUsages Örneği'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // API anahtarı için bir önek ayarlamak üzere aşağıdaki satırın yorumunu kaldırın, ör. "Token" (varsayılan null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    Double yearNumber = 3.4D; // Double | 
    Double monthNumber = 3.4D; // Double | 
    Double dayNumber = 3.4D; // Double | 
    Double skip = 3.4D; // Double | 
    try {
      GetTenantDailyUsages200Response result = apiInstance.getTenantDailyUsages(tenantId)
            .yearNumber(yearNumber)
            .monthNumber(monthNumber)
            .dayNumber(dayNumber)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getTenantDailyUsages");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---