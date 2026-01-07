Here we'll walk through calling the FastComments API to setup access control.

Before we begin, note that we don't have to explicitly create a `Group` structure. Groups are simply identifiers
added to `Users` and `Pages`. Adding a group to a user or page automatically "creates" the group.

First, let's create two users, `User A` and `User B`, we'll start them out in `Group X`:

[inline-code-attrs-start title = 'Create User A cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Create User B cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Now let's create a `Page`. We'll call it our `Confidential Page`, and so far none of these users will have access to it as it will
be in the group `CONFIDENTIAL`:

[inline-code-attrs-start title = 'Page POST cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Users A and B currently **DO NOT** have access to the new page. However, since they are in the same group, `GROUP-X`, they can `@mention` each other.

Let's update `User B` so they can now access the page:

[inline-code-attrs-start title = 'Update User B cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` now belongs to both groups. Our users can still `@mention` each other, but only `User B` can view our confidential page.

Let's make it so `User B` can only view the confidential page:

[inline-code-attrs-start title = 'Update User B cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Now they can view the confidential page, but neither of our users can `@mention` each other, as they are in different groups.

However, any user that is not part of access control **will be able to access our page**. To prevent this, ensure no SSO Users have
their `groupIds` set to null. For example, let's create `User C`, which has access to everything:

[inline-code-attrs-start title = 'Create User C cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

By setting `groupIds` to null, we say they are not limited by access control.

Now, let's create a page that everyone has access to:

[inline-code-attrs-start title = 'Page POST cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

By setting `accessibleByGroupIds` to null, we say this `Page` is not controlled via access control, and both users can access it.

This completes our API walk-through for Access Control.
