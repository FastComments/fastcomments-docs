Habilitar o deshabilitar las notificaciones para una página. Cuando los usuarios están suscritos a una página, se crean notificaciones
para nuevos comentarios raíz, y también

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| urlId | string | query | Sí |  |
| url | string | query | Sí |  |
| pageTitle | string | query | Sí |  |
| subscribedOrUnsubscribed | string | path | Sí |  |
| sso | string | query | No |  |

## Response

Devuelve: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/UpdateUserNotificationStatus200Response.java)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateUserNotificationPageSubscriptionStatus'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String url = "url_example"; // String | 
    String pageTitle = "pageTitle_example"; // String | 
    String subscribedOrUnsubscribed = "subscribe"; // String | 
    String sso = "sso_example"; // String | 
    try {
      UpdateUserNotificationStatus200Response result = apiInstance.updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscribedOrUnsubscribed)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Excepción al llamar a PublicApi#updateUserNotificationPageSubscriptionStatus");
      System.err.println("Código de estado: " + e.getCode());
      System.err.println("Razón: " + e.getResponseBody());
      System.err.println("Encabezados de la respuesta: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]