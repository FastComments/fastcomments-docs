## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| text-search | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/ModerationSuggestResponse.java)

## Exemple

[inline-code-attrs-start title = 'Exemple getSearchSuggest'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer des classes :
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
    String textSearch = "textSearch_example"; // String |
    String sso = "sso_example"; // String |
    try {
      ModerationSuggestResponse result = apiInstance.getSearchSuggest(tenantId)
            .textSearch(textSearch)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#getSearchSuggest");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---