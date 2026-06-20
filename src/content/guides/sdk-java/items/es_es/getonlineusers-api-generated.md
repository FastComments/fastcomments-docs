Espectadores actualmente en línea de una página: personas cuya sesión websocket está suscrita a la página en este momento.
Devuelve anonCount + totalCount (suscriptores de la sala en general, incluyendo espectadores anónimos que no enumeramos).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| urlId | string | query | Sí | Identificador de URL de la página (limpiado en el servidor). |
| afterName | string | query | No | Cursor: pase nextAfterName de la respuesta anterior. |
| afterUserId | string | query | No | Desempate de cursor: pase nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates por nombre no omitan entradas. |

## Response

Devuelve: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## Example

[inline-code-attrs-start title = 'Ejemplo de getOnlineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importar clases:
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
    String urlId = "urlId_example"; // String | Identificador de URL de la página (limpiado en el servidor).
    String afterName = "afterName_example"; // String | Cursor: pase nextAfterName de la respuesta anterior.
    String afterUserId = "afterUserId_example"; // String | Desempate de cursor: pase nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates por nombre no omitan entradas.
    try {
      PageUsersOnlineResponse result = apiInstance.getOnlineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOnlineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]