Los votos individuales detrás de los totales de una encuesta, del más antiguo al más reciente.

Una encuesta pertenece a un comentario, por lo que los votos siempre se leen de una encuesta a la vez - commentId es obligatorio. Esto
mantiene cada consulta en los índices que la colección ya tiene.

Respeta la privacidad de la encuesta: los votos de una encuesta anónima no se pueden leer (poll-anonymous), ni aquí ni por id.

## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| commentId | string | query | Sí |  |
| voterId | string | query | No |  |
| optionId | string | query | No |  |
| skip | integer | query | No |  |

## Respuesta

Devuelve: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPollVotesResponse.java)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getPollVotes'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Configurar la autorización con clave de API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Descomente la siguiente línea para establecer un prefijo para la clave de API, p. ej. "Token" (por defecto es null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    String voterId = "voterId_example"; // String | 
    String optionId = "optionId_example"; // String | 
    Integer skip = 56; // Integer | 
    try {
      GetPollVotesResponse result = apiInstance.getPollVotes(tenantId, commentId)
            .voterId(voterId)
            .optionId(optionId)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Excepción al llamar a DefaultApi#getPollVotes");
      System.err.println("Código de estado: " + e.getCode());
      System.err.println("Motivo: " + e.getResponseBody());
      System.err.println("Encabezados de respuesta: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]