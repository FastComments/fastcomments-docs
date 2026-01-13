### שימוש ב-APIs מאומתים (DefaultApi)

**Important:** עליך להגדיר את ה-API key ב-ApiClient לפני ביצוע בקשות מאומתות. אם לא תעשה זאת, הבקשות ייכשלו עם שגיאת 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // צור וקנפג את ApiClient
        ApiClient apiClient = new ApiClient();

        // REQUIRED: קבע את ה-API key שלך (קבל אותו מלוח הבקרה של FastComments)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // צור את מופע ה-API עם ה-ApiClient שהוגדר
        DefaultApi api = new DefaultApi(apiClient);

        // עכשיו אתה יכול לבצע קריאות API מאומתות
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
            // שגיאות נפוצות:
            // - 401: ה-API key חסר או לא תקף
            // - 400: אימות הבקשה נכשל
        }
    }
}
```

### שימוש ב-APIs ציבוריים (PublicApi)

נקודות קצה ציבוריות אינן דורשות אימות:

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

### בעיות נפוצות

1. **401 "missing-api-key" error**: ודא שאתה קורא ל-`apiClient.setApiKey("YOUR_KEY")` לפני יצירת המופע של DefaultApi.
2. **מחלקת API שגויה**: השתמש ב-`DefaultApi` עבור בקשות מאומתות בצד השרת, וב-`PublicApi` עבור בקשות בצד הלקוח/ציבוריות.
3. **Null API key**: ה-SDK יתעלם בשקט מאימות אם ה-API key הוא null, מה שיוביל לשגיאות 401.