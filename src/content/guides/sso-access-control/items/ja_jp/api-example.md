ここでは FastComments API を呼び出してアクセス制御を設定する方法を説明します。

始める前に、`Group` 構造を明示的に作成する必要はないことに注意してください。Groups は単に `Users` と `Pages` に追加される識別子です。ユーザーやページにグループを追加すると、そのグループは自動的に「作成」されます。

まず、`User A` と `User B` の2人のユーザーを作成します。最初は両方とも `Group X` に入れます：

[inline-code-attrs-start title = 'User A を作成する cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'User B を作成する cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

次に `Page` を作成します。これを `Confidential Page` と呼びます。現時点ではこれらのユーザーは誰もこのページにアクセスできません。なぜならこのページは `CONFIDENTIAL` グループに属しているからです：

[inline-code-attrs-start title = 'ページ POST cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

`User A` と `User B` は現時点で新しいページに **アクセスできません**。しかし、両者は同じグループ `GROUP-X` にいるため、お互いを `@mention` することはできます。

`User B` を更新して、ページにアクセスできるようにしましょう：

[inline-code-attrs-start title = 'User B を更新する cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` は現在両方のグループに属しています。ユーザー同士は引き続き `@mention` できますが、閲覧できるのは `User B` だけです。

`User B` が機密ページのみ閲覧できるようにしましょう：

[inline-code-attrs-start title = 'User B を更新する cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

これで `User B` は機密ページを閲覧できますが、両ユーザーは異なるグループにいるため互いに `@mention` することはできません。

しかし、アクセス制御の一部でないユーザーは**ページにアクセスできてしまいます**。これを防ぐには、SSO ユーザーの `groupIds` が null に設定されていないことを確認してください。例えば、すべてにアクセスできる `User C` を作成してみます：

[inline-code-attrs-start title = 'User C を作成する cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

`groupIds` を null に設定すると、そのユーザーはアクセス制御の制限を受けないことを示します。

次に、全員がアクセスできるページを作成します：

[inline-code-attrs-start title = 'ページ POST cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

`accessibleByGroupIds` を null に設定すると、この `Page` はアクセス制御の対象外となり、両方のユーザーがアクセスできるようになります。

これでアクセス制御のための API 実演は完了です。