## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | yol | Evet |  |
| broadcastId | string | sorgu | Hayır |  |
| sso | string | sorgu | Hayır |  |

## Yanıt

Döndürür: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateFeedPostResponse.java)

## Örnek

[inline-code-attrs-start title = 'createFeedPostPublic Örneği'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    CreateFeedPostParams createFeedPostParams = new CreateFeedPostParams(); // CreateFeedPostParams | 
    String broadcastId = "broadcastId_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      CreateFeedPostResponse result = apiInstance.createFeedPostPublic(tenantId, createFeedPostParams)
            .broadcastId(broadcastId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("PublicApi#createFeedPostPublic çağrılırken istisna oluştu");
      System.err.println("Durum kodu: " + e.getCode());
      System.err.println("Sebep: " + e.getResponseBody());
      System.err.println("Yanıt başlıkları: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]