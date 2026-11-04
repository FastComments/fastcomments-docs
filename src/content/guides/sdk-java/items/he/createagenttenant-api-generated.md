יוצר חשבון ניסיון חדש עבור סוכן AI ללא הרשמה של בן אדם. אין צורך במפתח API כדי לקרוא לזה.

התשובה מכילה את מזהה הטננט, מפתח API שעובד מיד מול ה-REST API ושרת ה-MCP, וכתובת URL לתביעה. מסרו את כתובת ה-URL לתביעה לבן האדם שאתם עובדים עבורו: פתיחתה בזמן שמחוברים ל-FastComments מקשרת את החשבון אליו. חשבונות שלא נתבעו, והמפתחות שלהם, נמחקים 72 שעות לאחר היצירה. עד לתביעה, לחשבון יש את מגבלות הניסיון הרגילות.

## תשובה

מחזיר: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## דוגמה

[inline-code-attrs-start title = 'דוגמת createAgentTenant'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    CreateAgentTenantBody createAgentTenantBody = new CreateAgentTenantBody(); // CreateAgentTenantBody | 
    try {
      CreateAgentTenantResponse result = apiInstance.createAgentTenant(createAgentTenantBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createAgentTenant");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]