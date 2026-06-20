---
רשימת דפים עבור שוכר. משמשת את לקוח שולחן העבודה של FChat כדי למלא את רשימת החדרים שלו.
נדרש ש-`enableFChat` יהיה true בתצורת ה-custom המתקבלת עבור כל דף.
דפים שדורשים SSO מסוננים בהתאם לגישת הקבוצות של המשתמש המבקש.

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | סמן דפדוף עמום המוחזר כ-`nextCursor` מבקשה קודמת. קשור לאותו `sortBy`. |
| limit | integer | query | No | 1..200, ברירת מחדל 50 |
| q | string | query | No | פילטר אופציונלי לקידומת כותרת שאינה תלויה ברישיות. |
| sortBy | string | query | No | סדר מיון. `updatedAt` (ברירת מחדל, החדשים ביותר ראשונים), `commentCount` (הדפים עם הכי הרבה תגובות ראשונים), או `title` (א-ב). |
| hasComments | boolean | query | No | אם true — להחזיר רק דפים עם לפחות תגובה אחת. |

## תגובה

מחזיר: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getPagesPublic'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// ייבוא מחלקות:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.PublicApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    PublicApi apiInstance = new PublicApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String cursor = "cursor_example"; // String | סמן דפדוף עמום המוחזר כ-`nextCursor` מבקשה קודמת. קשור לאותו `sortBy`.
    Integer limit = 56; // Integer | 1..200, ברירת מחדל 50
    String q = "q_example"; // String | פילטר אופציונלי לקידומת כותרת שאינה תלויה ברישיות.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | סדר מיון. `updatedAt` (ברירת מחדל, החדשים ביותר ראשונים), `commentCount` (הדפים עם הכי הרבה תגובות ראשונים), או `title` (א-ב).
    Boolean hasComments = true; // Boolean | אם true — להחזיר רק דפים עם לפחות תגובה אחת.
    try {
      GetPublicPagesResponse result = apiInstance.getPagesPublic(tenantId)
            .cursor(cursor)
            .limit(limit)
            .q(q)
            .sortBy(sortBy)
            .hasComments(hasComments)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getPagesPublic");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---