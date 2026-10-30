## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | El inquilino del que leer los registros de auditoría. |
| limit | integer | query | No | Cuántos registros devolver. Máximo 10000, por defecto 1000. |
| skip | integer | query | No | Cuántos registros omitir, para paginación. |
| order | string | query | No | Dirección de ordenación sobre la hora del evento. Por defecto DESC (más reciente primero). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Sólo devolver eventos realizados por este nombre de usuario. |
| ip | string | query | No | Sólo devolver eventos de esta dirección IP. |
| crudType | string | query | No | Sólo devolver eventos de este tipo: c, r, u, d, o login. |
| resourceName | string | query | No | Sólo devolver eventos para este recurso, p. ej. User o Moderator. |
| targetId | string | query | No | Sólo devolver eventos cuyo objeto afectado tenga este id. |
| target | string | query | No | Coincidencia de subcadena sin distinción de mayúsculas/minúsculas en la etiqueta del objeto afectado, p. ej. un nombre de usuario o correo electrónico. |
| includeManagedTenants | boolean | query | No | También devolver eventos de los inquilinos que este inquilino gestiona. |

## Respuesta

Devuelve: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getAuditLogs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
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
    
    // Configure API key authorization: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Uncomment the following line to set a prefix for the API key, e.g. "Token" (defaults to null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | El inquilino del que leer los registros de auditoría.
    Integer limit = 56; // Integer | Cuántos registros devolver. Máximo 10000, por defecto 1000.
    Integer skip = 56; // Integer | Cuántos registros omitir, para paginación.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | Dirección de ordenación sobre la hora del evento. Por defecto DESC (más reciente primero).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | Sólo devolver eventos realizados por este nombre de usuario.
    String ip = "ip_example"; // String | Sólo devolver eventos de esta dirección IP.
    String crudType = "crudType_example"; // String | Sólo devolver eventos de este tipo: c, r, u, d, o login.
    String resourceName = "resourceName_example"; // String | Sólo devolver eventos para este recurso, p. ej. User o Moderator.
    String targetId = "targetId_example"; // String | Sólo devolver eventos cuyo objeto afectado tenga este id.
    String target = "target_example"; // String | Coincidencia de subcadena sin distinción de mayúsculas/minúsculas en la etiqueta del objeto afectado, p. ej. un nombre de usuario o correo electrónico.
    Boolean includeManagedTenants = true; // Boolean | También devolver eventos de los inquilinos que este inquilino gestiona.
    try {
      GetAuditLogsResponse result = apiInstance.getAuditLogs(tenantId)
            .limit(limit)
            .skip(skip)
            .order(order)
            .after(after)
            .before(before)
            .username(username)
            .ip(ip)
            .crudType(crudType)
            .resourceName(resourceName)
            .targetId(targetId)
            .target(target)
            .includeManagedTenants(includeManagedTenants)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getAuditLogs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]