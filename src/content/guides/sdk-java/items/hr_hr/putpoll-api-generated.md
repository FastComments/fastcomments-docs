Priložite anketu komentaru ili postavite cijelo stanje ankete koju već ima.

Opcije se podudaraju po id: opcija poslana s id-em postojeće opcije zadržava svoje glasove (i preuzima novu oznaku i poziciju), opcija poslana bez id-a se dodaje, a postojeće opcije izostavljene iz liste se uklanjaju zajedno s glasovima danima na njima.

Ako na anketi koja ima glasove ne zadržite nijedan id postojeće opcije, svi glasovi se brišu, pa tada treba replaceVotes=true.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| commentId | string | path | Da |  |
| replaceVotes | boolean | query | Ne |  |

## Odgovor

Vraća: [`SavePollResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/SavePollResponse.java)

## Primjer

[inline-code-attrs-start title = 'putPoll primjer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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