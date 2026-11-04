İnsan kaydı olmadan bir AI ajanı için yeni bir deneme hesabı oluşturur. Bu çağrı için API anahtarı gerekmez.

Yanıt, tenant id, REST API ve MCP sunucusuna karşı hemen çalışabilen bir API anahtarı ve bir claim URL içerir. Claim URL'sini sizin için çalışan insana verin: FastComments'a giriş yapmışken açtığında hesap ona bağlanır. Talep edilmemiş (unclaimed) hesaplar ve anahtarları, oluşturulduktan 72 saat sonra silinir. Talep edilene kadar hesap standart deneme limitlerine sahiptir.

## Yanıt

Döndürür: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## Örnek

[inline-code-attrs-start title = 'createAgentTenant Örneği'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    CreateAgentTenantBody createAgentTenantBody = new CreateAgentTenantBody(); // CreateAgentTenantBody | 
    try {
      CreateAgentTenantResponse result = apiInstance.createAgentTenant(createAgentTenantBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createAgentTenant");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]