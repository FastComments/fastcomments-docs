Sayfadaki, şu anda çevrimiçi olmayan önceki yorumcular. displayName'e göre sıralanır.
Bu, bir "Üyeler" bölümü oluşturmak için /users/online tükendiğinde kullanın.
commenterName üzerinde imleçli sayfalama: sunucu, kısmi {tenantId, urlId, commenterName} indeksinde afterName'den başlayarak $gt ile ileri doğru tarar, $skip maliyeti yok.

## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir). |
| afterName | string | query | No | İmleç: önceki yanıttan nextAfterName değerini gönderin. |
| afterUserId | string | query | No | İmleç eşitlik kırıcı: önceki yanıttan nextAfterUserId değerini gönderin. afterName ayarlandığında, aynı ada sahip girişlerin düşmemesi için gereklidir. |

## Yanıt

Döndürür: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## Örnek

[inline-code-attrs-start title = 'getOfflineUsers Örneği'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String afterName = "afterName_example"; // String | İmleç: önceki yanıttan nextAfterName değerini gönderin.
    String afterUserId = "afterUserId_example"; // String | İmleç eşitlik kırıcı: önceki yanıttan nextAfterUserId değerini gönderin. afterName ayarlandığında, aynı ada sahip girişlerin düşmemesi için gereklidir.
    try {
      PageUsersOfflineResponse result = apiInstance.getOfflineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOfflineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]