### Using Authenticated APIs (DefaultApi)

**Important:** You must set your API key on the ApiClient before making authenticated requests. If you don't, requests will fail with a 401 error.

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

### Using Public APIs (PublicApi)

Public endpoints don't require authentication:

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

### Common Issues

1. **401 "missing-api-key" error**: Make sure you call `apiClient.setApiKey("YOUR_KEY")` before creating the DefaultApi instance.
2. **Wrong API class**: Use `DefaultApi` for server-side authenticated requests, `PublicApi` for client-side/public requests.
3. **Null API key**: The SDK will silently skip authentication if the API key is null, leading to 401 errors.