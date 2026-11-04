Vedhæft en afstemning til en kommentar, eller angiv den fulde tilstand for den afstemning, den allerede har.

Svarmuligheder matches ud fra id: en mulighed, der sendes med id'et for en eksisterende mulighed, bevarer sine stemmer (og får den nye label og position), en mulighed, der sendes uden id, tilføjes, og eksisterende muligheder, der udelades fra listen, fjernes sammen med de stemmer, der er afgivet på dem.

Hvis man beholder ingen eksisterende muligheds-id'er på en afstemning, der har stemmer, slettes alle af dem, så det kræver replaceVotes=true.

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |
| replaceVotes | boolean | query | Nej |  |

## Svar

Returnerer: [`SavePollResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/SavePollResponse.java)

## Eksempel

[inline-code-attrs-start title = 'putPoll Eksempel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
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
    
    // Configure API key authorization: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Uncomment the following line to set a prefix for the API key, e.g. "Token" (defaults to null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    CommentPollPutInput commentPollPutInput = new CommentPollPutInput(); // CommentPollPutInput | 
    Boolean replaceVotes = true; // Boolean | 
    try {
      SavePollResponse result = apiInstance.putPoll(tenantId, commentId, commentPollPutInput)
            .replaceVotes(replaceVotes)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#putPoll");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]