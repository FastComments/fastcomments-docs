req
tenantId
urlId

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | pot | Da |  |
| urlId | string | poizvedba | Da |  |
| page | integer | poizvedba | Ne |  |
| direction | string | poizvedba | Ne |  |
| sso | string | poizvedba | Ne |  |
| skip | integer | poizvedba | Ne |  |
| skipChildren | integer | poizvedba | Ne |  |
| limit | integer | poizvedba | Ne |  |
| limitChildren | integer | poizvedba | Ne |  |
| countChildren | boolean | poizvedba | Ne |  |
| fetchPageForCommentId | string | poizvedba | Ne |  |
| includeConfig | boolean | poizvedba | Ne |  |
| countAll | boolean | poizvedba | Ne |  |
| includei10n | boolean | poizvedba | Ne |  |
| locale | string | poizvedba | Ne |  |
| modules | string | poizvedba | Ne |  |
| isCrawler | boolean | poizvedba | Ne |  |
| includeNotificationCount | boolean | poizvedba | Ne |  |
| asTree | boolean | poizvedba | Ne |  |
| maxTreeDepth | integer | poizvedba | Ne |  |
| useFullTranslationIds | boolean | poizvedba | Ne |  |
| parentId | string | poizvedba | Ne |  |
| searchText | string | poizvedba | Ne |  |
| hashTags | array | poizvedba | Ne |  |
| userId | string | poizvedba | Ne |  |
| customConfigStr | string | poizvedba | Ne |  |
| afterCommentId | string | poizvedba | Ne |  |
| beforeCommentId | string | poizvedba | Ne |  |

## Odgovor

Vrne: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetCommentsPublic200Response.java)

## Primer

[inline-code-attrs-start title = 'Primer getCommentsPublic'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvozi razrede:
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
    String urlId = "urlId_example"; // String | 
    Integer page = 56; // Integer | 
    SortDirections direction = SortDirections.fromValue("OF"); // SortDirections | 
    String sso = "sso_example"; // String | 
    Integer skip = 56; // Integer | 
    Integer skipChildren = 56; // Integer | 
    Integer limit = 56; // Integer | 
    Integer limitChildren = 56; // Integer | 
    Boolean countChildren = true; // Boolean | 
    String fetchPageForCommentId = "fetchPageForCommentId_example"; // String | 
    Boolean includeConfig = true; // Boolean | 
    Boolean countAll = true; // Boolean | 
    Boolean includei10n = true; // Boolean | 
    String locale = "locale_example"; // String | 
    String modules = "modules_example"; // String | 
    Boolean isCrawler = true; // Boolean | 
    Boolean includeNotificationCount = true; // Boolean | 
    Boolean asTree = true; // Boolean | 
    Integer maxTreeDepth = 56; // Integer | 
    Boolean useFullTranslationIds = true; // Boolean | 
    String parentId = "parentId_example"; // String | 
    String searchText = "searchText_example"; // String | 
    List<String> hashTags = Arrays.asList(); // List<String> | 
    String userId = "userId_example"; // String | 
    String customConfigStr = "customConfigStr_example"; // String | 
    String afterCommentId = "afterCommentId_example"; // String | 
    String beforeCommentId = "beforeCommentId_example"; // String | 
    try {
      GetCommentsPublic200Response result = apiInstance.getCommentsPublic(tenantId, urlId)
            .page(page)
            .direction(direction)
            .sso(sso)
            .skip(skip)
            .skipChildren(skipChildren)
            .limit(limit)
            .limitChildren(limitChildren)
            .countChildren(countChildren)
            .fetchPageForCommentId(fetchPageForCommentId)
            .includeConfig(includeConfig)
            .countAll(countAll)
            .includei10n(includei10n)
            .locale(locale)
            .modules(modules)
            .isCrawler(isCrawler)
            .includeNotificationCount(includeNotificationCount)
            .asTree(asTree)
            .maxTreeDepth(maxTreeDepth)
            .useFullTranslationIds(useFullTranslationIds)
            .parentId(parentId)
            .searchText(searchText)
            .hashTags(hashTags)
            .userId(userId)
            .customConfigStr(customConfigStr)
            .afterCommentId(afterCommentId)
            .beforeCommentId(beforeCommentId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getCommentsPublic");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]