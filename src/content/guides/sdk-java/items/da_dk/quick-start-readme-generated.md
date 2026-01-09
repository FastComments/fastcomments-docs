### Brug af autentificerede API'er (DefaultApi)

**Vigtigt:** Du skal angive din API-nøgle på ApiClient før du foretager autentificerede anmodninger. Hvis du ikke gør det, vil anmodninger mislykkes med en 401-fejl.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Opret og konfigurer API-klienten
        ApiClient apiClient = new ApiClient();

        // PÅKRÆVET: Indstil din API-nøgle (hent den fra dit FastComments-dashboard)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Opret API-instanse med den konfigurerede klient
        DefaultApi api = new DefaultApi(apiClient);

        // Nu kan du lave autentificerede API-kald
        try {
            // Eksempel: Tilføj en SSO-bruger
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Almindelige fejl:
            // - 401: API-nøgle mangler eller er ugyldig
            // - 400: Anmodningsvalidering mislykkedes
        }
    }
}
```

### Brug af offentlige API'er (PublicApi)

Offentlige endpoints kræver ikke autentificering:

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

### Almindelige problemer

1. **401 "missing-api-key" fejl**: Sørg for at du kalder `apiClient.setApiKey("YOUR_KEY")` før du opretter DefaultApi-instanse.
2. **Forkert API-klasse**: Brug `DefaultApi` for server-side autentificerede anmodninger, `PublicApi` for klient-side/offentlige anmodninger.
3. **Null API-nøgle**: SDK'en vil stiltiende springe autentificering over, hvis API-nøglen er null, hvilket fører til 401-fejl.