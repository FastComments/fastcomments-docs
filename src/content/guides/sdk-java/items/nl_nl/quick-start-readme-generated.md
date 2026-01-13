### Geauthenticeerde API's gebruiken (DefaultApi)

**Belangrijk:** u moet uw API-sleutel op de ApiClient instellen voordat u geauthenticeerde aanvragen doet. Als u dat niet doet, zullen aanvragen mislukken met een 401-fout.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Maak en configureer de API-client
        ApiClient apiClient = new ApiClient();

        // VEREIST: Stel uw API-sleutel in (verkrijg deze via uw FastComments-dashboard)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Maak de API-instantie met de geconfigureerde client
        DefaultApi api = new DefaultApi(apiClient);

        // Nu kunt u geauthenticeerde API-aanroepen doen
        try {
            // Voorbeeld: voeg een SSO-gebruiker toe
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Veelvoorkomende fouten:
            // - 401: API-sleutel ontbreekt of is ongeldig
            // - 400: Validatie van het verzoek is mislukt
        }
    }
}
```

### Openbare API's gebruiken (PublicApi)

Openbare endpoints vereisen geen authenticatie:

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

### Veelvoorkomende problemen

1. **401 "missing-api-key" fout**: Zorg ervoor dat u `apiClient.setApiKey("YOUR_KEY")` aanroept voordat u de DefaultApi-instantie maakt.
2. **Verkeerde API-klasse**: Gebruik `DefaultApi` voor server-side geauthenticeerde verzoeken, `PublicApi` voor client-side/openbare verzoeken.
3. **Null API-sleutel**: De SDK zal authenticatie stilzwijgend overslaan als de API-sleutel null is, wat leidt tot 401-fouten.