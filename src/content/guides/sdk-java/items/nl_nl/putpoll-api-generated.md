Attach een poll aan een reactie, of stel de volledige status van de poll in die deze al heeft.

Opties worden gematcht op id: een optie die wordt verzonden met de id van een bestaande optie behoudt zijn stemmen (en neemt het nieuwe label en de positie), een optie die zonder id wordt verzonden wordt toegevoegd, en bestaande opties die niet in de lijst staan worden verwijderd samen met de daarop uitgebrachte stemmen.

Het niet behouden van bestaande optie‑id's op een poll die stemmen heeft, verwijdert ze allemaal, dus moet replaceVotes=true worden gebruikt.

## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |
| replaceVotes | boolean | query | Nee |  |

## Respons

Retourneert: [`SavePollResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/SavePollResponse.java)

## Voorbeeld

[inline-code-attrs-start title = 'putPoll Voorbeeld'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importeer klassen:
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
    
    // Configureer API-sleutel autorisatie: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Verwijder commentaar van de volgende regel om een prefix voor de API-sleutel in te stellen, bijv. "Token" (standaard null)
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

---