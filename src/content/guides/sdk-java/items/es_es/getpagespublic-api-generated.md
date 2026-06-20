Lista páginas para un tenant. Usado por el cliente de escritorio FChat para poblar su lista de salas. Requiere `enableFChat` sea true en la configuración personalizada resuelta para cada página. Las páginas que requieren SSO se filtran según el acceso de grupo del usuario solicitante.

## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Cursor de paginación opaco devuelto como `nextCursor` de una solicitud anterior. Vinculado al mismo `sortBy`. |
| limit | integer | query | No | 1..200, por defecto 50 |
| q | string | query | No | Filtro opcional por prefijo de título, sin distinguir mayúsculas/minúsculas. |
| sortBy | string | query | No | Orden. `updatedAt` (por defecto, recientes primero), `commentCount` (más comentarios primero) o `title` (alfabético). |
| hasComments | boolean | query | No | Si es true, devuelve solo páginas con al menos un comentario. |

## Respuesta

Devuelve: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getPagesPublic'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String cursor = "cursor_example"; // String | Cursor de paginación opaco devuelto como `nextCursor` de una solicitud anterior. Vinculado al mismo `sortBy`.
    Integer limit = 56; // Integer | 1..200, por defecto 50
    String q = "q_example"; // String | Filtro opcional por prefijo de título, sin distinguir mayúsculas/minúsculas.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | Orden. `updatedAt` (por defecto, recientes primero), `commentCount` (más comentarios primero), o `title` (alfabético).
    Boolean hasComments = true; // Boolean | Si es true, devuelve solo páginas con al menos un comentario.
    try {
      GetPublicPagesResponse result = apiInstance.getPagesPublic(tenantId)
            .cursor(cursor)
            .limit(limit)
            .q(q)
            .sortBy(sortBy)
            .hasComments(hasComments)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getPagesPublic");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]