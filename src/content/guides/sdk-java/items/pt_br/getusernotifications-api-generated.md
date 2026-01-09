## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| pageSize | integer | query | Não |  |
| afterId | string | query | Não |  |
| includeContext | boolean | query | Não |  |
| afterCreatedAt | integer | query | Não |  |
| unreadOnly | boolean | query | Não |  |
| dmOnly | boolean | query | Não |  |
| noDm | boolean | query | Não |  |
| includeTranslations | boolean | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetUserNotifications200Response.java)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getUserNotifications'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importar classes:
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
    Integer pageSize = 56; // Integer | 
    String afterId = "afterId_example"; // String | 
    Boolean includeContext = true; // Boolean | 
    Long afterCreatedAt = 56L; // Long | 
    Boolean unreadOnly = true; // Boolean | 
    Boolean dmOnly = true; // Boolean | 
    Boolean noDm = true; // Boolean | 
    Boolean includeTranslations = true; // Boolean | 
    String sso = "sso_example"; // String | 
    try {
      GetUserNotifications200Response result = apiInstance.getUserNotifications(tenantId)
            .pageSize(pageSize)
            .afterId(afterId)
            .includeContext(includeContext)
            .afterCreatedAt(afterCreatedAt)
            .unreadOnly(unreadOnly)
            .dmOnly(dmOnly)
            .noDm(noDm)
            .includeTranslations(includeTranslations)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getUserNotifications");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---