List pages for a tenant. Used by the FChat desktop client to populate its room list.
Vereist dat `enableFChat` op true staat in de opgeloste aangepaste configuratie voor elke pagina.
Pagina's die SSO vereisen worden gefilterd op basis van de groepstoegang van de aanvragende gebruiker.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| cursor | string | query | Nee | Opaak pagineringscursor teruggegeven als `nextCursor` van een eerdere aanvraag. Gebonden aan dezelfde `sortBy`. |
| limit | integer | query | Nee | 1..200, standaard 50 |
| q | string | query | Nee | Optionele hoofdletterongevoelige titel-voorvoegselfilter. |
| sortBy | string | query | Nee | Sorteervolgorde. `updatedAt` (standaard, nieuwste eerst), `commentCount` (meeste opmerkingen eerst), of `title` (alfabetisch). |
| hasComments | boolean | query | Nee | Als true, geef alleen pagina's terug met ten minste één opmerking. |

## Response

Geeft terug: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## Voorbeeld

[inline-code-attrs-start title = 'getPagesPublic Voorbeeld'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importeer klassen:
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
    String cursor = "cursor_example"; // String | Opaak pagineringscursor teruggegeven als `nextCursor` van een eerdere aanvraag. Gebonden aan dezelfde `sortBy`.
    Integer limit = 56; // Integer | 1..200, standaard 50
    String q = "q_example"; // String | Optionele hoofdletterongevoelige titel-voorvoegselfilter.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | Sorteervolgorde. `updatedAt` (standaard, nieuwste eerst), `commentCount` (meeste opmerkingen eerst), of `title` (alfabetisch).
    Boolean hasComments = true; // Boolean | Als true, geef alleen pagina's terug met ten minste één opmerking.
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

---