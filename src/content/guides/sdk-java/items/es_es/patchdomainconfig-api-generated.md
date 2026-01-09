## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| domainToUpdate | string | path | Yes |  |

## Respuesta

Devuelve: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetDomainConfig200Response.java)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de patchDomainConfig'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Configurar autorización por clave API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Descomente la siguiente línea para establecer un prefijo para la clave API, p. ej. "Token" (por defecto null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String domainToUpdate = "domainToUpdate_example"; // String | 
    PatchDomainConfigParams patchDomainConfigParams = new PatchDomainConfigParams(); // PatchDomainConfigParams | 
    try {
      GetDomainConfig200Response result = apiInstance.patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Excepción al llamar a DefaultApi#patchDomainConfig");
      System.err.println("Código de estado: " + e.getCode());
      System.err.println("Razón: " + e.getResponseBody());
      System.err.println("Encabezados de respuesta: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]