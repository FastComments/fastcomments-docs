Bir kiracı için sayfaları listeler. FChat masaüstü istemcisi tarafından oda listesini doldurmak için kullanılır.
Her sayfa için çözümlenen özel yapılandırmada `enableFChat`'in true olmasını gerektirir.
SSO gerektiren sayfalar, istekte bulunan kullanıcının grup erişimine göre filtrelenir.

## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Önceki bir istekte `nextCursor` olarak döndürülen opak sayfalama imleci. Aynı `sortBy` ile ilişkilidir. |
| limit | integer | query | No | 1..200, varsayılan 50 |
| q | string | query | No | İsteğe bağlı, büyük/küçük harf duyarsız başlık öneki filtresi. |
| sortBy | string | query | No | Sıralama düzeni. `updatedAt` (varsayılan, en yeniler önce), `commentCount` (en çok yorum önce), veya `title` (alfabetik). |
| hasComments | boolean | query | No | Eğer true ise, yalnızca en az bir yorumu olan sayfaları döndürür. |

## Yanıt

Döndürür: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## Örnek

[inline-code-attrs-start title = 'getPagesPublic Örneği'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String cursor = "cursor_example"; // String | Önceki bir istekte `nextCursor` olarak döndürülen opak sayfalama imleci. Aynı `sortBy` ile ilişkilidir.
    Integer limit = 56; // Integer | 1..200, varsayılan 50
    String q = "q_example"; // String | İsteğe bağlı, büyük/küçük harf duyarsız başlık öneki filtresi.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | Sıralama düzeni. `updatedAt` (varsayılan, en yeniler önce), `commentCount` (en çok yorum önce), veya `title` (alfabetik).
    Boolean hasComments = true; // Boolean | Eğer true ise, yalnızca en az bir yorumu olan sayfaları döndürür.
    try {
      GetPublicPagesResponse result = apiInstance.getPagesPublic(tenantId)
            .cursor(cursor)
            .limit(limit)
            .q(q)
            .sortBy(sortBy)
            .hasComments(hasComments)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getPagesPublic");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]