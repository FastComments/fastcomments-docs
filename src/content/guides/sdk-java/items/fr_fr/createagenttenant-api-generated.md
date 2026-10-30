Crée un nouveau compte d’essai pour un agent IA sans inscription humaine. Aucun clé d’API n’est nécessaire pour appeler ceci.

La réponse contient l’identifiant du locataire, une clé d’API qui fonctionne immédiatement contre l’API REST et le serveur MCP, ainsi qu’une URL de réclamation. Donnez l’URL de réclamation à la personne pour qui vous travaillez : l’ouvrir tout en étant connecté à FastComments attache le compte à cette personne. Les comptes non réclamés, et leurs clés, sont supprimés 72 heures après leur création. Jusqu’à réclamation, le compte a les limites d’essai standard.

## Réponse

Retourne : [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## Exemple

[inline-code-attrs-start title = 'Exemple createAgentTenant'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer les classes :
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    CreateAgentTenantBody createAgentTenantBody = new CreateAgentTenantBody(); // CreateAgentTenantBody | 
    try {
      CreateAgentTenantResponse result = apiInstance.createAgentTenant(createAgentTenantBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createAgentTenant");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---