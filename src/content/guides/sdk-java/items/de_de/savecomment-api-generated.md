## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| isLive | boolean | query | Nein |  |
| doSpamCheck | boolean | query | Nein |  |
| sendEmails | boolean | query | Nein |  |
| populateNotifications | boolean | query | Nein |  |

## Antwort

Gibt zurück: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/APISaveCommentResponse.java)

## Beispiel

[inline-code-attrs-start title = 'saveComment Beispiel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Klassen importieren:
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
    
    // API-Schlüssel-Authentifizierung konfigurieren: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Kommentieren Sie die folgende Zeile aus, um ein Präfix für den API-Schlüssel zu setzen, z. B. "Token" (Standardmäßig null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    CreateCommentParams createCommentParams = new CreateCommentParams(); // CreateCommentParams | 
    Boolean isLive = true; // Boolean | 
    Boolean doSpamCheck = true; // Boolean | 
    Boolean sendEmails = true; // Boolean | 
    Boolean populateNotifications = true; // Boolean | 
    try {
      APISaveCommentResponse result = apiInstance.saveComment(tenantId, createCommentParams)
            .isLive(isLive)
            .doSpamCheck(doSpamCheck)
            .sendEmails(sendEmails)
            .populateNotifications(populateNotifications)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#saveComment");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]