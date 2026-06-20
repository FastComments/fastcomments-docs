Listet Seiten für einen Mandanten. Vom FChat-Desktop-Client verwendet, um seine Raumliste zu füllen. Erfordert, dass `enableFChat` im aufgelösten benutzerdefinierten Konfigurationsobjekt für jede Seite auf true gesetzt ist. Seiten, die SSO erfordern, werden gegen die Gruppenberechtigung des anfragenden Benutzers gefiltert.

## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Opaker Paginierungs-Cursor, der als `nextCursor` aus einer vorherigen Anfrage zurückgegeben wurde. Dem gleichen `sortBy` zugeordnet. |
| limit | integer | query | No | 1..200, Standard 50 |
| q | string | query | No | Optionaler, groß-/kleinschreibungsunabhängiger Präfixfilter für den Titel. |
| sortBy | string | query | No | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst) oder `title` (alphabetisch). |
| hasComments | boolean | query | No | Wenn true, nur Seiten zurückgeben, die mindestens einen Kommentar enthalten. |

## Antwort

Gibt zurück: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getPagesPublic'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
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
    String cursor = "cursor_example"; // String | Opaker Paginierungs-Cursor, der als `nextCursor` aus einer vorherigen Anfrage zurückgegeben wurde. Dem gleichen `sortBy` zugeordnet.
    Integer limit = 56; // Integer | 1..200, Standard 50
    String q = "q_example"; // String | Optionaler, groß-/kleinschreibungsunabhängiger Präfixfilter für den Titel.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst), oder `title` (alphabetisch).
    Boolean hasComments = true; // Boolean | Wenn true, nur Seiten zurückgeben, die mindestens einen Kommentar haben.
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