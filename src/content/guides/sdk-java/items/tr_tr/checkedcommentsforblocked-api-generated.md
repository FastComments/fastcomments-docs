## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentIds | string | query | Yes | Yorum kimliklerinin virgülle ayrılmış listesi. |
| sso | string | query | No |  |

## Yanıt

Döndürür: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CheckBlockedCommentsResponse.java)

## Örnek

[inline-code-attrs-start title = 'checkedCommentsForBlocked Örneği'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sınıfları içe aktar:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.PublicApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    PublicApi apiInstance = new PublicApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentIds = "commentIds_example"; // String | Yorum kimliklerinin virgülle ayrılmış listesi.
    String sso = "sso_example"; // String | 
    try {
      CheckBlockedCommentsResponse result = apiInstance.checkedCommentsForBlocked(tenantId, commentIds)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("PublicApi#checkedCommentsForBlocked çağrılırken oluşan istisna");
      System.err.println("Durum kodu: " + e.getCode());
      System.err.println("Sebep: " + e.getResponseBody());
      System.err.println("Yanıt üstbilgileri: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---