### Χρήση Πιστοποιημένων API (DefaultApi)

**Σημαντικό:** Πρέπει να ορίσετε το API key σας στο ApiClient πριν κάνετε πιστοποιημένα αιτήματα. Αν δεν το κάνετε, τα αιτήματα θα αποτύχουν με σφάλμα 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Δημιουργήστε και ρυθμίστε τον API client
        ApiClient apiClient = new ApiClient();

        // ΑΠΑΡΑΙΤΗΤΟ: Ορίστε το API key σας (το βγάζετε από το ταμπλό του FastComments)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Δημιουργήστε την υλοποίηση του API με τον ρυθμισμένο client
        DefaultApi api = new DefaultApi(apiClient);

        // Τώρα μπορείτε να κάνετε πιστοποιημένα κλήσεις API
        try {
            // Παράδειγμα: Προσθήκη χρήστη SSO
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Συνηθισμένα σφάλματα:
            // - 401: Το API key λείπει ή είναι άκυρο
            // - 400: Η επικύρωση του αιτήματος απέτυχε
        }
    }
}
```

### Χρήση Δημόσιων API (PublicApi)

Οι δημόσιες διαδρομές (endpoints) δεν απαιτούν πιστοποίηση:

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

### Χρήση API Εποπτείας (ModerationApi)

Το `ModerationApi` τροφοδοτεί το πάνελ του moderator. Κάθε μέθοδος δέχεται μια παράμετρο `sso` που αναγνωρίζει τον moderator πιστοποιημένο με SSO εκ μέρους του οποίου γίνεται το αίτημα:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // Λίστα σχολίων που αναμένουν εποπτεία
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Συνηθισμένα Προβλήματα

1. **401 "missing-api-key" σφάλμα**: Βεβαιωθείτε ότι καλείτε `apiClient.setApiKey("YOUR_KEY")` πριν δημιουργήσετε το instance του DefaultApi.
2. **Λανθασμένη κλάση API**: Χρησιμοποιήστε `DefaultApi` για server-side πιστοποιημένα αιτήματα, `PublicApi` για client-side/δημόσια αιτήματα.
3. **Μηδενικό API key**: Το SDK θα παραλείψει σιωπηλά την πιστοποίηση αν το API key είναι null, οδηγώντας σε σφάλματα 401.