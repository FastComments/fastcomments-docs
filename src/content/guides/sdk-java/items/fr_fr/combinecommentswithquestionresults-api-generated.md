---
## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| questionId | string | query | Non |  |
| questionIds | array | query | Non |  |
| urlId | string | query | Non |  |
| startDate | string | query | Non |  |
| forceRecalculate | boolean | query | Non |  |
| minValue | number | query | Non |  |
| maxValue | number | query | Non |  |
| limit | number | query | Non |  |

## Réponse

Renvoie : [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CombineCommentsWithQuestionResults200Response.java)

## Exemple

[inline-code-attrs-start title = 'Exemple de combineCommentsWithQuestionResults'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer les classes :
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
    
    // Configurer l'autorisation par clé API : api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Décommentez la ligne suivante pour définir un préfixe pour la clé API, p.ex. "Token" (par défaut à null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String questionId = "questionId_example"; // String | 
    List<String> questionIds = Arrays.asList(); // List<String> | 
    String urlId = "urlId_example"; // String | 
    OffsetDateTime startDate = OffsetDateTime.now(); // OffsetDateTime | 
    Boolean forceRecalculate = true; // Boolean | 
    Double minValue = 3.4D; // Double | 
    Double maxValue = 3.4D; // Double | 
    Double limit = 3.4D; // Double | 
    try {
      CombineCommentsWithQuestionResults200Response result = apiInstance.combineCommentsWithQuestionResults(tenantId)
            .questionId(questionId)
            .questionIds(questionIds)
            .urlId(urlId)
            .startDate(startDate)
            .forceRecalculate(forceRecalculate)
            .minValue(minValue)
            .maxValue(maxValue)
            .limit(limit)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#combineCommentsWithQuestionResults");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---