## Parametri

| Name | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| isLive | boolean | query | No |  |
| doSpamCheck | boolean | query | No |  |
| sendEmails | boolean | query | No |  |
| populateNotifications | boolean | query | No |  |

## Risposta

Restituisce: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/APISaveCommentResponse.java)

## Esempio

[inline-code-attrs-start title = 'Esempio di saveComment'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importa le classi:
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
    
    // Configura l'autenticazione tramite chiave API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Rimuovi il commento dalla seguente riga per impostare un prefisso per la chiave API, es. "Token" (di default null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // Stringa | 
    CreateCommentParams createCommentParams = new CreateCommentParams(); // CreateCommentParams | 
    Boolean isLive = true; // Booleano | 
    Boolean doSpamCheck = true; // Booleano | 
    Boolean sendEmails = true; // Booleano | 
    Boolean populateNotifications = true; // Booleano | 
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