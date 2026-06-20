## Parameters

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| forceRecalculate | boolean | query | No |  |

## Response

Restituisce: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/BulkAggregateQuestionResultsResponse.java)

## Esempio

[inline-code-attrs-start title = 'Esempio di bulkAggregateQuestionResults'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Configura l'autorizzazione con API key: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Decommentare la seguente riga per impostare un prefisso per la API key, es. "Token" (predefinito null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    BulkAggregateQuestionResultsRequest bulkAggregateQuestionResultsRequest = new BulkAggregateQuestionResultsRequest(); // BulkAggregateQuestionResultsRequest | 
    Boolean forceRecalculate = true; // Boolean | 
    try {
      BulkAggregateQuestionResultsResponse result = apiInstance.bulkAggregateQuestionResults(tenantId, bulkAggregateQuestionResultsRequest)
            .forceRecalculate(forceRecalculate)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#bulkAggregateQuestionResults");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]