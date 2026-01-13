在此我們將說明如何呼叫 FastComments API 以設定存取控制。

在開始之前，注意我們不需要明確建立 `Group` 結構。群組只是加到 `Users` 與 `Pages` 的識別符。將群組新增到使用者或頁面會自動「建立」該群組。

首先，讓我們建立兩個使用者，`User A` 與 `User B`，並將他們一開始放在 `Group X`：

[inline-code-attrs-start title = '建立 User A 的 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = '建立 User B 的 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

現在讓我們建立一個 `Page`。我們稱它為 `Confidential Page`，目前這些使用者都無法存取，因為該頁面屬於群組 `CONFIDENTIAL`：

[inline-code-attrs-start title = '頁面 POST cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

使用者 A 與 B 目前**無法**存取該新頁面。然而，因為他們在同一個群組 `GROUP-X`，他們仍然可以互相 `@mention`。

讓我們更新 `User B`，使他們可以存取該頁面：

[inline-code-attrs-start title = '更新 User B 的 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` 現在屬於兩個群組。我們的使用者仍然可以互相 `@mention`，但只有 `User B` 可以查看我們的機密頁面。

讓 `User B` 只能查看機密頁面：

[inline-code-attrs-start title = '更新 User B 的 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

現在他們可以查看機密頁面，但我們的兩位使用者都無法互相 `@mention`，因為他們屬於不同的群組。

然而，任何未納入存取控制的使用者**仍然能夠存取我們的頁面**。為了避免這種情況，請確保沒有 SSO 使用者的 `groupIds` 被設為 null。例如，讓我們建立 `User C`，他可以存取所有內容：

[inline-code-attrs-start title = '建立 User C 的 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

透過將 `groupIds` 設為 null，我們表示他們不受存取控制限制。

現在，讓我們建立一個每個人都能存取的頁面：

[inline-code-attrs-start title = '頁面 POST cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

透過將 `accessibleByGroupIds` 設為 null，我們表示此 `Page` 不受存取控制，而兩位使用者皆可存取。

此 API 存取控制操作說明到此結束。