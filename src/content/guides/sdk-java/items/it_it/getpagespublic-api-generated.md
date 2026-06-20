Elenca le pagine per un tenant. Utilizzato dal client desktop FChat per popolare l'elenco delle sue stanze.
Richiede che `enableFChat` sia true sulla configurazione personalizzata risolta per ogni pagina.
Le pagine che richiedono SSO sono filtrate in base all'accesso per gruppi dell'utente che effettua la richiesta.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Cursore di paginazione opaco restituito come `nextCursor` da una richiesta precedente. Legato allo stesso `sortBy`. |
| limit | integer | query | No | 1..200, predefinito 50 |
| q | string | query | No | Filtro opzionale per prefisso del titolo, senza distinzione tra maiuscole e minuscole. |
| sortBy | string | query | No | Ordinamento. `updatedAt` (predefinito, i più recenti prima), `commentCount` (i più commentati prima), o `title` (alfabetico). |
| hasComments | boolean | query | No | Se true, restituisce solo le pagine con almeno un commento. |

## Risposta

Restituisce: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## Esempio

[inline-code-attrs-start title = 'Esempio di getPagesPublic'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importa classi:
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
    String cursor = "cursor_example"; // String | Cursore di paginazione opaco restituito come `nextCursor` da una richiesta precedente. Legato allo stesso `sortBy`.
    Integer limit = 56; // Integer | 1..200, predefinito 50
    String q = "q_example"; // String | Filtro opzionale per prefisso del titolo, senza distinzione tra maiuscole e minuscole.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | Ordinamento. `updatedAt` (predefinito, i più recenti prima), `commentCount` (i più commentati prima), o `title` (alfabetico).
    Boolean hasComments = true; // Boolean | Se true, restituisce solo le pagine con almeno un commento.
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