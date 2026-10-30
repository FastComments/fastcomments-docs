Bir sayfa için beğeni sayısı, isteğe bağlı olarak beğenen kullanıcılarla birlikte.

## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| urlId | string | query | Evet |  |
| includeUserInfo | string | query | Hayır | "public" özel profillere sahip kullanıcıları hariç tutar, "include-private" onları dahil eder. |

## Yanıt

Döndürür: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPageLikesAPIResponse.java)

## Örnek

[inline-code-attrs-start title = 'getPageLikes Örneği'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | 
    PageReactIncludeUserInfo includeUserInfo = PageReactIncludeUserInfo.fromValue("public"); // PageReactIncludeUserInfo | "public" özel profillere sahip kullanıcıları hariç tutar, "include-private" onları dahil eder.
    try {
      GetPageLikesAPIResponse result = apiInstance.getPageLikes(tenantId, urlId)
            .includeUserInfo(includeUserInfo)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getPageLikes");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]