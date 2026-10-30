Crea una nueva cuenta de prueba para un agente de IA sin que un humano se registre. No se necesita una clave API para llamar a este.

La respuesta contiene el ID del inquilino, una clave API que funciona inmediatamente contra la API REST y el servidor MCP, y una URL de reclamación. Proporcione la URL de reclamación a la persona para la que está trabajando: abrirla mientras está conectado a FastComments asocia la cuenta a ella. Las cuentas no reclamadas, y sus claves, se eliminan 72 horas después de su creación. Hasta que se reclamen, la cuenta tiene los límites de prueba estándar.

## Response

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## Example

[inline-code-attrs-start title = 'Ejemplo createAgentTenant'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importar clases:
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

---