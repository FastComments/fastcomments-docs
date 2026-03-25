## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| id | string | path | Sí |  |
| userId | string | query | No |  |

## Respuesta

Devuelve: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/UpdateSubscriptionAPIResponse.java)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateSubscription'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importar clases:
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
    
    // Configurar autorización con clave API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Descomente la siguiente línea para establecer un prefijo para la clave API, p. ej. "Token" (por defecto null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String id = "id_example"; // String | 
    UpdateAPIUserSubscriptionData updateAPIUserSubscriptionData = new UpdateAPIUserSubscriptionData(); // UpdateAPIUserSubscriptionData | 
    String userId = "userId_example"; // String | 
    try {
      UpdateSubscriptionAPIResponse result = apiInstance.updateSubscription(tenantId, id, updateAPIUserSubscriptionData)
            .userId(userId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#updateSubscription");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]