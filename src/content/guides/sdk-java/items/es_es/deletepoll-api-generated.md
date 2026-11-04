Elimina la encuesta de su comentario, junto con todos los votos emitidos en ella. El comentario en sí queda intacto.

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| commentId | string | path | Sí |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/APIEmptyResponse.java)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deletePoll'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Configurar la autorización mediante clave de API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Descomenta la siguiente línea para establecer un prefijo para la clave de API, p. ej. "Token" (por defecto es null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    try {
      APIEmptyResponse result = apiInstance.deletePoll(tenantId, commentId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Excepción al llamar a DefaultApi#deletePoll");
      System.err.println("Código de estado: " + e.getCode());
      System.err.println("Motivo: " + e.getResponseBody());
      System.err.println("Cabeceras de respuesta: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]