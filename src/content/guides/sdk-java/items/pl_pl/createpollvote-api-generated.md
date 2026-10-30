Record a vote on a poll, or move an existing one to a different option. A voter has at most one vote per
poll, so calling this again for the same voter moves their vote rather than adding one.

This obeys the site's poll settings: if voting is set to logged-in users only, a vote with just an
anonUserId is rejected, and anonymous votes are rate limited per IP per poll.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Returns: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreatePollVoteResponse.java)

## Example

[inline-code-attrs-start title = 'Przykład createPollVote'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importuj klasy:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.auth.*;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");
    
    // Skonfiguruj autoryzację klucza API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Odkomentuj poniższą linię, aby ustawić prefiks dla klucza API, np. "Token" (domyślnie null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    CreatePollVoteBody createPollVoteBody = new CreatePollVoteBody(); // CreatePollVoteBody | 
    try {
      CreatePollVoteResponse result = apiInstance.createPollVote(tenantId, createPollVoteBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Wyjątek podczas wywoływania DefaultApi#createPollVote");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]