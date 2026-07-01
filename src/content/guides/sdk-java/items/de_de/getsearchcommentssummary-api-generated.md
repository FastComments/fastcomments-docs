## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|-----|--------------|--------------|
| tenantId | string | query | Ja |  |
| value | string | query | Nein |  |
| filters | string | query | Nein |  |
| searchFilters | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Rückgabe: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/ModerationCommentSearchResponse.java)

## Beispiel

[inline-code-attrs-start title = 'getSearchCommentsSummary Beispiel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importiere Klassen:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.ModerationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    ModerationApi apiInstance = new ModerationApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String value = "value_example"; // String | 
    String filters = "filters_example"; // String | 
    String searchFilters = "searchFilters_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      ModerationCommentSearchResponse result = apiInstance.getSearchCommentsSummary(tenantId)
            .value(value)
            .filters(filters)
            .searchFilters(searchFilters)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Ausnahme beim Aufruf von ModerationApi#getSearchCommentsSummary");
      System.err.println("Statuscode: " + e.getCode());
      System.err.println("Grund: " + e.getResponseBody());
      System.err.println("Antwort-Header: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]