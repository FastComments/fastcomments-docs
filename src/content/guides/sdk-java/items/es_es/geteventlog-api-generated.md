req
tenantId
urlId
userIdWS

## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | ruta | Sí |  |
| urlId | string | consulta | Sí |  |
| userIdWS | string | consulta | Sí |  |
| startTime | integer | consulta | Sí |  |
| endTime | integer | consulta | No |  |

## Respuesta

Devuelve: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetEventLogResponse.java)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getEventLog'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | 
    String userIdWS = "userIdWS_example"; // String | 
    Long startTime = 56L; // Long | 
    Long endTime = 56L; // Long | 
    try {
      GetEventLogResponse result = apiInstance.getEventLog(tenantId, urlId, userIdWS, startTime)
            .endTime(endTime)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getEventLog");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---