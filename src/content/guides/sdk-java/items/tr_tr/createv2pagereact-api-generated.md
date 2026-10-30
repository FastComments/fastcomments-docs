## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| id | string | query | Yes |  |
| title | string | query | No |  |
| sso | string | query | No |  |

## Yanıt

Döndürür: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateV1PageReact.java)

## Örnek

[inline-code-attrs-start title = 'createV2PageReact Örneği'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | 
    String id = "id_example"; // String | 
    String title = "title_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      CreateV1PageReact result = apiInstance.createV2PageReact(tenantId, urlId, id)
            .title(title)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("PublicApi#createV2PageReact çağrılırken istisna");
      System.err.println("Durum kodu: " + e.getCode());
      System.err.println("Sebep: " + e.getResponseBody());
      System.err.println("Yanıt başlıkları: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---