Registra un voto en una encuesta, o mueve uno existente a una opción diferente. Un votante tiene como máximo un voto por
encuesta, por lo que llamar a esto de nuevo para el mismo votante mueve su voto en lugar de añadir uno.

Esto respeta la configuración de encuestas del sitio: si la votación está configurada solo para usuarios conectados, un voto con solo un
anonUserId es rechazado, y los votos anónimos tienen límites de frecuencia por IP por encuesta.

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |

## Respuesta

Devuelve: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreatePollVoteResponse.java)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createPollVote'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Descomenta la siguiente línea para establecer un prefijo para la clave de API, p. ej. "Token" (por defecto es null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    CreatePollVoteBody createPollVoteBody = new CreatePollVoteBody(); // CreatePollVoteBody | 
    try {
      CreatePollVoteResponse result = apiInstance.createPollVote(tenantId, createPollVoteBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Excepción al llamar a DefaultApi#createPollVote");
      System.err.println("Código de estado: " + e.getCode());
      System.err.println("Motivo: " + e.getResponseBody());
      System.err.println("Cabeceras de respuesta: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]