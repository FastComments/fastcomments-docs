Edita una encuesta en el lugar, manteniendo sus recuentos: cambia la pregunta, vuelve a etiquetar una opción, ciérrala o reábrela, o cambia quién puede ver a los votantes. Las opciones se abordan por id; para agregar, eliminar o reordenarlas, haz un PUT de la lista completa.

## Parameters

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |

## Response

Devuelve: [`SavePollResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/SavePollResponse.java)

## Example

[inline-code-attrs-start title = 'Ejemplo patchPoll'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Configurar autorización de clave API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Descomente la siguiente línea para establecer un prefijo para la clave API, por ejemplo "Token" (por defecto es null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    CommentPollPatch commentPollPatch = new CommentPollPatch(); // CommentPollPatch | 
    try {
      SavePollResponse result = apiInstance.patchPoll(tenantId, commentId, commentPollPatch)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#patchPoll");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---