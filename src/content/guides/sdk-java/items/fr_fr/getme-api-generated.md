Identifie les informations d’identification utilisées : le locataire auquel elles appartiennent et, pour les jetons OAuth, l’utilisateur qui les a autorisées.  
Les intégrations utilisent cela pour tester une connexion et l’étiqueter.

## Parameters

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |

## Response

Renvoie : [`GetMeResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetMeResponse.java)

## Example

[inline-code-attrs-start title = 'Exemple getMe'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer les classes :
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
    
    // Configurer l'autorisation de la clé API : api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Décommentez la ligne suivante pour définir un préfixe pour la clé API, par ex. "Token" (valeur par défaut null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    try {
      GetMeResponse result = apiInstance.getMe(tenantId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      // Exception lors de l'appel de DefaultApi#getMe
      System.err.println("Exception when calling DefaultApi#getMe");
      System.err.println("Code d'état : " + e.getCode());
      System.err.println("Raison : " + e.getResponseBody());
      System.err.println("En-têtes de réponse : " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---