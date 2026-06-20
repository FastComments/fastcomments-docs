### Brug af autentificerede API'er (DefaultApi)

**Vigtigt:** Du skal angive din API-nøgle på ApiClient før du foretager autentificerede forespørgsler. Hvis du ikke gør det, vil forespørgsler fejle med en 401-fejl.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Opret og konfigurer API-klienten
        ApiClient apiClient = new ApiClient();

        // PÅKRÆVET: Angiv din API-nøgle (hent den fra dit FastComments-dashboard)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Opret API-instansen med den konfigurerede klient
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
            // - 400: Validering af forespørgsel mislykkedes
        }
    }
}
```

### Brug af offentlige API'er (PublicApi)

Offentlige endepunkter kræver ikke autentificering:

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

### Brug af Moderation APIs (ModerationApi)

The `ModerationApi` drives the moderator dashboard. Each method accepts an `sso` parameter identifying the SSO-authenticated moderator on whose behalf the request is made:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // List kommentarer, der afventer moderation
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Almindelige problemer

1. **401 "missing-api-key" fejl**: Sørg for, at du kalder `apiClient.setApiKey("YOUR_KEY")` før du opretter DefaultApi-instansen.  
2. **Forkert API-klasse**: Brug `DefaultApi` til server-side autentificerede forespørgsler, `PublicApi` til klient-side/offentlige forespørgsler.  
3. **Null API-nøgle**: SDK'en vil stilfærdigt springe autentificering over, hvis API-nøglen er null, hvilket fører til 401-fejl.