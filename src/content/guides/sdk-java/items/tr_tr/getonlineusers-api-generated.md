Bir sayfanın şu anda çevrimiçi izleyicileri: websocket oturumu şu anda sayfaya abone olan kişiler.
anonCount + totalCount döndürür (oda genelindeki aboneler; anonim izleyicileri dahil eder, ancak onları listelemiyoruz).

## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir). |
| afterName | string | query | No | İmleç: önceki yanıttan nextAfterName değerini iletin. |
| afterUserId | string | query | No | İmleç eşitlik kırıcı: önceki yanıttan nextAfterUserId değerini iletin. afterName ayarlandığında, isim bağları nedeniyle girişlerin düşmemesi için gereklidir. |

## Response

Döndürür: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## Örnek

[inline-code-attrs-start title = 'getOnlineUsers Örneği'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir).
    String afterName = "afterName_example"; // String | İmleç: önceki yanıttan nextAfterName değerini iletin.
    String afterUserId = "afterUserId_example"; // String | İmleç eşitlik kırıcı: önceki yanıttan nextAfterUserId değerini iletin. afterName ayarlandığında, isim bağları nedeniyle girişlerin düşmemesi için gereklidir.
    try {
      PageUsersOnlineResponse result = apiInstance.getOnlineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOnlineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]