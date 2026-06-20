### Gebruik van geauthenticeerde API's (DefaultApi)

**Belangrijk:** Je moet je API-sleutel instellen op de ApiClient voordat je geauthenticeerde verzoeken doet. Als je dat niet doet, zullen verzoeken mislukken met een 401-fout.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Create and configure the API client
        ApiClient apiClient = new ApiClient();

        // REQUIRED: Set your API key (get this from your FastComments dashboard)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Create the API instance with the configured client
        DefaultApi api = new DefaultApi(apiClient);

        // Now you can make authenticated API calls
        try {
            // Example: Add an SSO user
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Common errors:
            // - 401: API key is missing or invalid
            // - 400: Request validation failed
        }
    }
}
```

### Gebruik van publieke API's (PublicApi)

Publieke endpoints vereisen geen authenticatie:

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

### Gebruik van moderatie-API's (ModerationApi)

De `ModerationApi` stuurt het moderator-dashboard aan. Elke methode accepteert een `sso`-parameter die de SSO-geauthenticeerde moderator identificeert namens wie het verzoek wordt gedaan:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // List comments awaiting moderation
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Veelvoorkomende problemen

1. **401 "missing-api-key" error**: Zorg ervoor dat je `apiClient.setApiKey("YOUR_KEY")` aanroept voordat je de DefaultApi-instantie aanmaakt.
2. **Verkeerde API-klasse**: Gebruik `DefaultApi` voor server-side geauthenticeerde verzoeken, `PublicApi` voor client-side/publieke verzoeken.
3. **Null API key**: De SDK zal authenticatie stilzwijgend overslaan als de API-sleutel null is, wat leidt tot 401-fouten.