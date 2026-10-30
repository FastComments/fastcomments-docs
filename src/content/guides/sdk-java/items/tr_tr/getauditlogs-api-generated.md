## Parameters

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet | Denetim günlüklerini okunacak kiracı. |
| limit | integer | query | Hayır | Döndürülecek günlük sayısı. Maksimum 10000, varsayılan 1000. |
| skip | integer | query | Hayır | Sayfalama için atlanacak günlük sayısı. |
| order | string | query | Hayır | Olay zamanına göre sıralama yönü. Varsayılan DESC (en yeni önce). |
| after | integer | query | Hayır |  |
| before | integer | query | Hayır |  |
| username | string | query | Hayır | Yalnızca bu kullanıcı adıyla gerçekleştirilen olayları döndür. |
| ip | string | query | Hayır | Yalnızca bu IP adresinden gelen olayları döndür. |
| crudType | string | query | Hayır | Yalnızca bu türdeki olayları döndür: c, r, u, d veya giriş. |
| resourceName | string | query | Hayır | Yalnızca bu kaynak için olayları döndür, ör. Kullanıcı veya Moderatör. |
| targetId | string | query | Hayır | Yalnızca etkilenen nesnenin bu kimliğe sahip olduğu olayları döndür. |
| target | string | query | Hayır | Etkilenen nesnenin etiketinde büyük/küçük harfe duyarsız alt dize eşleşmesi, ör. bir kullanıcı adı veya e-posta. |
| includeManagedTenants | boolean | query | Hayır | Ayrıca bu kiracının yönettiği kiracılardan gelen olayları da döndür. |

## Response

Döndürür: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## Example

[inline-code-attrs-start title = 'getAuditLogs Örneği'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // API anahtarı için bir önek ayarlamak için aşağıdaki satırın yorumunu kaldırın, ör. "Token" (varsayılan null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | Denetim günlüklerini okunacak kiracı.
    Integer limit = 56; // Integer | Döndürülecek günlük sayısı. Maksimum 10000, varsayılan 1000.
    Integer skip = 56; // Integer | Sayfalama için atlanacak günlük sayısı.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | Olay zamanına göre sıralama yönü. Varsayılan DESC (en yeni önce).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | Yalnızca bu kullanıcı adıyla gerçekleştirilen olayları döndür.
    String ip = "ip_example"; // String | Yalnızca bu IP adresinden gelen olayları döndür.
    String crudType = "crudType_example"; // String | Yalnızca bu türdeki olayları döndür: c, r, u, d veya giriş.
    String resourceName = "resourceName_example"; // String | Yalnızca bu kaynak için olayları döndür, ör. Kullanıcı veya Moderatör.
    String targetId = "targetId_example"; // String | Yalnızca etkilenen nesnenin bu kimliğe sahip olduğu olayları döndür.
    String target = "target_example"; // String | Etkilenen nesnenin etiketinde büyük/küçük harfe duyarsız alt dize eşleşmesi, ör. bir kullanıcı adı veya e-posta.
    Boolean includeManagedTenants = true; // Boolean | Ayrıca bu kiracının yönettiği kiracılardan gelen olayları da döndür.
    try {
      GetAuditLogsResponse result = apiInstance.getAuditLogs(tenantId)
            .limit(limit)
            .skip(skip)
            .order(order)
            .after(after)
            .before(before)
            .username(username)
            .ip(ip)
            .crudType(crudType)
            .resourceName(resourceName)
            .targetId(targetId)
            .target(target)
            .includeManagedTenants(includeManagedTenants)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getAuditLogs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]