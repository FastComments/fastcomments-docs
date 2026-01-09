### Utiliser les API authentifiées (DefaultApi)

**Important :** Vous devez définir votre clé API sur ApiClient avant d'effectuer des requêtes authentifiées. Si vous ne le faites pas, les requêtes échoueront avec une erreur 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Créez et configurez le client API
        ApiClient apiClient = new ApiClient();

        // OBLIGATOIRE : Définissez votre clé API (récupérez-la depuis votre tableau de bord FastComments)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Créez l'instance API avec le client configuré
        DefaultApi api = new DefaultApi(apiClient);

        // Vous pouvez maintenant effectuer des appels API authentifiés
        try {
            // Exemple : ajouter un utilisateur SSO
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Erreurs courantes :
            // - 401 : la clé API est manquante ou invalide
            // - 400 : la validation de la requête a échoué
        }
    }
}
```

### Utiliser les API publiques (PublicApi)

Les endpoints publics ne nécessitent pas d'authentification :

```java
import com.fastcomments.api.PublicApi;
import com.fastcomments.invoker.ApiException;

PublicApi publicApi = new PublicApi();

try {
    var response = publicApi.getCommentsPublic("YOUR_TENANT_ID", "page-url-id")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Problèmes courants

1. **401 "missing-api-key" error** : Assurez-vous d'appeler `apiClient.setApiKey("YOUR_KEY")` avant de créer l'instance `DefaultApi`.
2. **Wrong API class** : Utilisez `DefaultApi` pour les requêtes authentifiées côté serveur, `PublicApi` pour les requêtes côté client/public.
3. **Null API key** : Le SDK ignorera silencieusement l'authentification si la clé API est nulle, ce qui entraînera des erreurs 401.