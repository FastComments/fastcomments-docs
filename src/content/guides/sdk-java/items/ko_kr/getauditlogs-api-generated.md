## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | 감사 로그를 읽을 테넌트. |
| limit | integer | query | No | 반환할 로그 수. 최대 10000, 기본값은 1000. |
| skip | integer | query | No | 페이지 매김을 위해 건너뛸 로그 수. |
| order | string | query | No | 이벤트 시간에 대한 정렬 방향. 기본값은 DESC (최신 순). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | 이 사용자 이름으로 수행된 이벤트만 반환합니다. |
| ip | string | query | No | 이 IP 주소에서 발생한 이벤트만 반환합니다. |
| crudType | string | query | No | 이 유형의 이벤트만 반환합니다: c, r, u, d, 또는 login. |
| resourceName | string | query | No | 이 리소스에 대한 이벤트만 반환합니다. 예: User 또는 Moderator. |
| targetId | string | query | No | 영향을 받은 객체에 이 ID가 있는 이벤트만 반환합니다. |
| target | string | query | No | 영향을 받은 객체 레이블에 대한 대소문자 구분 없는 부분 문자열 일치, 예: 사용자 이름 또는 이메일. |
| includeManagedTenants | boolean | query | No | 이 테넌트가 관리하는 테넌트의 이벤트도 반환합니다. |

## 응답

반환: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## 예시

[inline-code-attrs-start title = 'getAuditLogs 예시'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 클래스 가져오기:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.auth.*;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");
    
    // API 키 인증 구성: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // 다음 줄의 주석을 해제하여 API 키에 접두사를 설정합니다. 예: "Token" (기본값은 null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 감사 로그를 읽을 테넌트.
    Integer limit = 56; // Integer | 반환할 로그 수. 최대 10000, 기본값은 1000.
    Integer skip = 56; // Integer | 페이지 매김을 위해 건너뛸 로그 수.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | 이벤트 시간에 대한 정렬 방향. 기본값은 DESC (최신 순).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | 이 사용자 이름으로 수행된 이벤트만 반환합니다.
    String ip = "ip_example"; // String | 이 IP 주소에서 발생한 이벤트만 반환합니다.
    String crudType = "crudType_example"; // String | 이 유형의 이벤트만 반환합니다: c, r, u, d, 또는 login.
    String resourceName = "resourceName_example"; // String | 이 리소스에 대한 이벤트만 반환합니다. 예: User 또는 Moderator.
    String targetId = "targetId_example"; // String | 영향을 받은 객체에 이 ID가 있는 이벤트만 반환합니다.
    String target = "target_example"; // String | 영향을 받은 객체 레이블에 대한 대소문자 구분 없는 부분 문자열 일치, 예: 사용자 이름 또는 이메일.
    Boolean includeManagedTenants = true; // Boolean | 이 테넌트가 관리하는 테넌트의 이벤트도 반환합니다.
    try {
      GetAuditLogsResponse result = apiInstance.getAuditLogs(tenantId)
            .limit(limit)
            .skip(skip)
            .order(order)
            .after(after)
            .before(before)
            .username(username)
            .ip(ip)
            .crudType(crudType)
            .resourceName(resourceName)
            .targetId(targetId)
            .target(target)
            .includeManagedTenants(includeManagedTenants)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getAuditLogs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]