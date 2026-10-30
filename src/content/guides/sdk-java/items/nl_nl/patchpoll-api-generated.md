Edit een poll ter plekke, waarbij de tellingen behouden blijven: wijzig de vraag, hernoem een optie, sluit of heropen deze,
of wijzig wie de stemmers mag zien. Opties worden aangesproken via id – om ze toe te voegen, te verwijderen of te herschikken, PUT de volledige lijst.

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |

## Response

Retourneert: [`SavePollResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/SavePollResponse.java)

## Example

[inline-code-attrs-start title = 'patchPoll Voorbeeld'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Verwijder de commentaar van de volgende regel om een prefix voor de API-sleutel in te stellen, bijv. "Token" (standaard null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    CommentPollPatch commentPollPatch = new CommentPollPatch(); // CommentPollPatch | 
    try {
      SavePollResponse result = apiInstance.patchPoll(tenantId, commentId, commentPollPatch)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#patchPoll");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]