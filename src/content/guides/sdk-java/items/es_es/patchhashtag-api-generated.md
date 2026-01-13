## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tag | string | path | Sí |  |
| tenantId | string | query | No |  |

## Respuesta

Devuelve: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PatchHashTag200Response.java)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de patchHashTag'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Configurar la autorización por clave API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Descomenta la siguiente línea para establecer un prefijo para la clave API, p. ej. "Token" (por defecto null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tag = "tag_example"; // String | 
    String tenantId = "tenantId_example"; // String | 
    UpdateHashTagBody updateHashTagBody = new UpdateHashTagBody(); // UpdateHashTagBody | 
    try {
      PatchHashTag200Response result = apiInstance.patchHashTag(tag)
            .tenantId(tenantId)
            .updateHashTagBody(updateHashTagBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#patchHashTag");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---