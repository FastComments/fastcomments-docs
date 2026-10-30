The individual votes behind one poll's tallies, oldest first.

Un sondaggio appartiene a un commento, quindi i voti vengono sempre letti un sondaggio alla volta - **commentId** è obbligatorio. Questo mantiene ogni query sugli indici già presenti nella collezione.

Rispetta la privacy del sondaggio: i voti di un sondaggio anonimo non possono essere letti (poll-anonymous), né qui né tramite ID.

## Parameters

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|---------------|------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | Yes |  |
| voterId | string | query | No |  |
| optionId | string | query | No |  |
| skip | integer | query | No |  |

## Response

Restituisce: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPollVotesResponse.java)

## Esempio

[inline-code-attrs-start title = 'Esempio getPollVotes'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importa classi:
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
    
    // Configura l'autorizzazione della chiave API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Decommenta la riga seguente per impostare un prefisso per la chiave API, ad es. "Token" (predefinito a null)
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
      System.err.println("Eccezione durante la chiamata a DefaultApi#getPollVotes");
      System.err.println("Codice di stato: " + e.getCode());
      System.err.println("Motivo: " + e.getResponseBody());
      System.err.println("Intestazioni della risposta: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]