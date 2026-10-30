Ko je glasao za koju opciju. Anonimne ankete nikada ne odgovaraju, ankete samo za administratore odgovaraju samo za administratora ili moderatora.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| urlId | string | query | No |  |
| page | integer | query | No |  |
| search | string | query | No |  |
| sso | string | query | No |  |

## Response

Vraća: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPollVoterNamesSuccessResponse.java)

## Primer

[inline-code-attrs-start title = 'getPollVoters Primer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvezi klase:
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