Die einzelnen Stimmen hinter den Auszählungen einer Umfrage, älteste zuerst.

Eine Umfrage gehört zu einem Kommentar, daher werden Stimmen immer eine Umfrage nach der anderen gelesen – commentId ist erforderlich. Das hält jede Abfrage auf den bereits vorhandenen Indizes der Sammlung.

Beachtet die Privatsphäre der Umfrage: Die Stimmen einer anonymen Umfrage können nicht gelesen werden (poll-anonymous), weder hier noch per ID.

## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | query | Ja |  |
| voterId | string | query | Nein |  |
| optionId | string | query | Nein |  |
| skip | integer | query | Nein |  |

## Antwort

Rückgabe: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPollVotesResponse.java)

## Beispiel

[inline-code-attrs-start title = 'getPollVotes Beispiel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Konfiguriere API-Schlüssel-Authentifizierung: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Entferne das Auskommentieren der folgenden Zeile, um ein Präfix für den API-Schlüssel festzulegen, z. B. "Token" (Standard ist null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    String voterId = "voterId_example"; // String | 
    String optionId = "optionId_example"; // String | 
    Integer skip = 56; // Integer | 
    try {
      GetPollVotesResponse result = apiInstance.getPollVotes(tenantId, commentId)
            .voterId(voterId)
            .optionId(optionId)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getPollVotes");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]