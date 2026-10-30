Who voted for which option. Anonymous polls never answer, admins-only polls answer only for an admin or moderator.

## Parameters

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| commentId | string | path | Да |  |
| urlId | string | query | Не |  |
| page | integer | query | Не |  |
| search | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPollVoterNamesSuccessResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример getPollVoters'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импортиране на класове:
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