---
Fügen Sie einer Kommentar‑Umfrage eine Umfrage hinzu oder setzen Sie den vollständigen Zustand der bereits vorhandenen Umfrage.

Optionen werden anhand ihrer ID abgeglichen: Eine Option, die mit der ID einer bestehenden Option gesendet wird, behält ihre Stimmen (und übernimmt das neue Label und die Position), eine Option ohne ID wird hinzugefügt, und bestehende Optionen, die in der Liste ausgelassen werden, werden zusammen mit den darauf abgegebenen Stimmen entfernt.

Wenn bei einer Umfrage, die bereits Stimmen hat, keine bestehenden Options‑IDs beibehalten werden, werden alle gelöscht; daher muss replaceVotes=true gesetzt werden.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|-----|-----|---------------|----------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| replaceVotes | boolean | query | No |  |

## Antwort

Rückgabe: [`SavePollResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/SavePollResponse.java)

## Beispiel

[inline-code-attrs-start title = 'putPoll Beispiel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // API‑Schlüssel‑Authentifizierung konfigurieren: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Entfernen Sie das Kommentarzeichen in der folgenden Zeile, um ein Präfix für den API‑Schlüssel festzulegen, z. B. "Token" (Standard ist null)
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