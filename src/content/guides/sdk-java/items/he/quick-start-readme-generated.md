---
### שימוש ב-APIs מאומתים (DefaultApi)

**חשוב:** עליך להגדיר את מפתח ה-API ב-ApiClient לפני ביצוע בקשות מאומתות. אם לא תעשה זאת, הבקשות ייכשלו עם שגיאת 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // צור והגדר את לקוח ה-API
        ApiClient apiClient = new ApiClient();

        // REQUIRED: Set your API key (get this from your FastComments dashboard)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // צור את מופע ה-API עם הלקוח שהוגדר
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
            // - 401: מפתח ה-API חסר או לא תקין
            // - 400: אימות הבקשה נכשל
        }
    }
}
```

### שימוש ב-APIs ציבוריים (PublicApi)

נקודות קצה ציבוריות לא דורשות אימות:

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

### שימוש ב-APIs למודרציה (ModerationApi)

ה-`ModerationApi` מפעיל את דשבורד המודרטור. כל פעולה מקבלת פרמטר `sso` שמזהה את המודרטור שאומת דרך SSO ועבורו הבקשה מתבצעת:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // רשימת תגובות שממתינות למודרציה
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### בעיות נפוצות

1. **401 "missing-api-key" error**: ודא שאתה קורא ל-`apiClient.setApiKey("YOUR_KEY")` לפני יצירת מופע ה-DefaultApi.
2. **Wrong API class**: השתמש ב-`DefaultApi` לבקשות מאומתות בצד השרת, וב-`PublicApi` לבקשות בצד הלקוח/ציבוריות.
3. **Null API key**: ה-SDK ידלג על האימות בשקט אם מפתח ה-API הוא null, מה שיוביל לשגיאות 401.
---