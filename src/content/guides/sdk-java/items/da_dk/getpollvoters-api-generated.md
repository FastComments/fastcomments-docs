---
Hvem stemte på hvilken mulighed. Anonyme afstemninger svarer aldrig, admin‑kun afstemninger svarer kun for en administrator eller moderator.

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | path | Ja |  |
| commentId | string | path | Ja |  |
| urlId | string | query | Nej |  |
| page | integer | query | Nej |  |
| search | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPollVoterNamesSuccessResponse.java)

## Eksempel

[inline-code-attrs-start title = 'getPollVoters Eksempel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer klasser:
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
    String commentId = "commentId_example"; // String | 
    String urlId = "urlId_example"; // String | 
    Integer page = 56; // Integer | 
    String search = "search_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      GetPollVoterNamesSuccessResponse result = apiInstance.getPollVoters(tenantId, commentId)
            .urlId(urlId)
            .page(page)
            .search(search)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getPollVoters");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---