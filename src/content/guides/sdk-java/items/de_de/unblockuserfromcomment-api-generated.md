## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| userId | string | query | Nein |  |
| anonUserId | string | query | Nein |  |

## Antwort

Gibt zurück: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/UnBlockCommentPublic200Response.java)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für unBlockUserFromComment'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Entfernen Sie die Kommentarzeichen der folgenden Zeile, um ein Präfix für den API-Schlüssel festzulegen, z. B. "Token" (Standard: null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String id = "id_example"; // String | 
    UnBlockFromCommentParams unBlockFromCommentParams = new UnBlockFromCommentParams(); // UnBlockFromCommentParams | 
    String userId = "userId_example"; // String | 
    String anonUserId = "anonUserId_example"; // String | 
    try {
      UnBlockCommentPublic200Response result = apiInstance.unBlockUserFromComment(tenantId, id, unBlockFromCommentParams)
            .userId(userId)
            .anonUserId(anonUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#unBlockUserFromComment");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]