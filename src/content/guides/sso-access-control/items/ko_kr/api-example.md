여기서는 FastComments API를 호출하여 액세스 제어를 설정하는 과정을 안내합니다.

시작하기 전에, `Group` 구조를 명시적으로 생성할 필요가 없다는 점을 참고하세요. 그룹은 단순히 `Users`와 `Pages`에 추가되는 식별자입니다. 사용자나 페이지에 그룹을 추가하면 해당 그룹이 자동으로 "생성"됩니다.

먼저, 두 명의 사용자인 `User A`와 `User B`를 생성해 보겠습니다. 이들은 `Group X`에 속하도록 시작하겠습니다:

[inline-code-attrs-start title = '사용자 A 생성 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-a",
	"username": "User A",
	"email": "usera@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

[inline-code-attrs-start title = '사용자 B 생성 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-b",
	"username": "User B",
	"email": "userb@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

이제 `Page`를 하나 생성해 보겠습니다. 이를 `Confidential Page`라고 부르겠습니다. 현재 이 사용자들은 해당 페이지에 접근할 수 없습니다. 이 페이지는 `CONFIDENTIAL` 그룹에 속하기 때문입니다:

[inline-code-attrs-start title = '페이지 POST cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Confidential Page",
	"url": "https://mysite.com/confidential",
	"urlId": "https://mysite.com/confidential",
	"accessibleByGroupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

사용자 A와 B는 현재 새 페이지에 접근할 수 **없습니다**. 그러나 이들은 동일한 그룹인 `GROUP-X`에 속해 있으므로 서로 `@mention`할 수 있습니다.

이제 `User B`를 업데이트하여 해당 페이지에 접근할 수 있도록 해보겠습니다:

[inline-code-attrs-start title = '사용자 B 업데이트 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B`는 이제 두 그룹 모두에 속하게 되었습니다. 사용자들은 여전히 서로 `@mention`할 수 있지만, 기밀 페이지를 볼 수 있는 사용자는 `User B`뿐입니다.

이제 `User B`가 기밀 페이지만 볼 수 있도록 만들어 보겠습니다:

[inline-code-attrs-start title = '사용자 B 업데이트 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

이제 `User B`는 기밀 페이지를 볼 수 있지만, 두 사용자는 서로 다른 그룹에 속해 있으므로 서로 `@mention`할 수 없습니다.

그러나 액세스 제어에 포함되지 않은 사용자는 누구나 **페이지에 접근할 수 있습니다**. 이를 방지하려면 SSO 사용자의 `groupIds`가 null로 설정되지 않았는지 확인하세요. 예를 들어, 모든 것에 접근할 수 있는 `User C`를 생성해 보겠습니다:

[inline-code-attrs-start title = '사용자 C 생성 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-c",
	"username": "User C",
	"email": "userc@example.com",
    "groupIds": null
}'
[inline-code-end]

`groupIds`를 null로 설정하면 해당 사용자는 액세스 제어의 제한을 받지 않는다고 지정하는 것입니다.

이제 모든 사용자가 접근할 수 있는 페이지를 하나 만들어 보겠습니다:

[inline-code-attrs-start title = '페이지 POST cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Public Page",
	"url": "https://mysite.com/public",
	"urlId": "https://mysite.com/public",
	"accessibleByGroupIds": null
}'
[inline-code-end]

`accessibleByGroupIds`를 null로 설정하면 이 `Page`는 액세스 제어로 관리되지 않으며, 두 사용자 모두 접근할 수 있습니다.

이로써 액세스 제어에 대한 API 워크스루를 완료했습니다.

---