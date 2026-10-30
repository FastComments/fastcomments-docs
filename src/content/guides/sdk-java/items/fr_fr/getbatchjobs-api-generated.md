Les travaux batch du locataire (importations, exportations, migrations de pages...), du plus récent au plus ancien.

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|------------|------------|-------------|
| tenantId | string | query | Oui |  |
| type | string | query | Non | Uniquement les travaux de ce type, par exemple migrate-pages. |
| status | string | query | Non | Uniquement les travaux dans cet état : requested, running, success ou failed. |
| skip | integer | query | Non | Combien de travaux à ignorer, pour la pagination. |

## Réponse

Retourne : [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetBatchJobsAPIResponse.java)

## Exemple

[inline-code-attrs-start title = 'Exemple getBatchJobs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer les classes:
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
    
    // Configurer l'autorisation de la clé API : api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Décommentez la ligne suivante pour définir un préfixe pour la clé API, par ex. "Token" (valeur par défaut : null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String type = "type_example"; // String | Uniquement les travaux de ce type, par exemple migrate-pages.
    String status = "status_example"; // String | Uniquement les travaux dans cet état : requested, running, success ou failed.
    Integer skip = 56; // Integer | Combien de travaux à ignorer, pour la pagination.
    try {
      GetBatchJobsAPIResponse result = apiInstance.getBatchJobs(tenantId)
            .type(type)
            .status(status)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      // Exception lors de l'appel à DefaultApi#getBatchJobs
      System.err.println("Exception when calling DefaultApi#getBatchJobs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]