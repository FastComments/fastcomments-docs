### Brug af autentificerede API'er (DefaultApi)

**Vigtigt:** Du skal angive din API-nøgle på ApiClient, før du foretager autentificerede kald. Hvis du ikke gør det, vil kaldene fejle med en 401-fejl.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Opret og konfigurer API-klienten
        ApiClient apiClient = new ApiClient();

        // PÅKRÆVET: Angiv din API-nøgle (få den fra dit FastComments-dashboard)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Opret API-instansen med den konfigurerede klient
        DefaultApi api = new DefaultApi(apiClient);

        // Nu kan du foretage autentificerede API-kald
        try {
            // Eksempel: Tilføj en SSO-bruger
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("Bruger oprettet: " + response);

        } catch (ApiException e) {
            System.err.println("Fejl: " + e.getResponseBody());
            // Almindelige fejl:
            // - 401: API-nøglen mangler eller er ugyldig
            // - 400: Validering af anmodningen fejlede
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

### Brug af moderations-API'er (ModerationApi)

`ModerationApi` driver moderator-dashboardet. Hver metode accepterer en `sso`-parameter, der identificerer den SSO-autentificerede moderator, på hvis vegne anmodningen foretages:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // Vis kommentarer, der afventer moderation
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Almindelige problemer

1. **401 "missing-api-key"-fejl**: Sørg for at kalde `apiClient.setApiKey("YOUR_KEY")`, før du opretter DefaultApi-instansen.
2. **Forkert API-klasse**: Brug `DefaultApi` til autentificerede server-side-kald og `PublicApi` til klient-side/offentlige kald.
3. **Null API-nøgle**: SDK'et vil stille springe autentificering over, hvis API-nøglen er null, hvilket fører til 401-fejl.