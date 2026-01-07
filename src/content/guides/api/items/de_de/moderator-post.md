[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Diese Route bietet die Möglichkeit, einen einzelnen `Moderator` hinzuzufügen.

Das Erstellen eines `Moderator` hat folgende Einschränkungen:

- Ein `name` und eine `email` müssen immer angegeben werden. Eine `userId` ist optional.
- Die folgenden Werte dürfen beim Erstellen eines `Moderator` nicht angegeben werden:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Wenn eine `userId` angegeben wird, muss dieser Benutzer existieren.
- Wenn eine `userId` angegeben wird, muss sie zum selben `tenantId` gehören, der in den Abfrageparametern angegeben ist.
- Zwei Moderatoren im selben Tenant können nicht mit derselben `email` hinzugefügt werden.

Wir können einen `Moderator` für einen Benutzer erstellen, von dem wir nur die E-Mail kennen:

[inline-code-attrs-start title = 'Moderator Erstellen via E-Mail cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

Oder wir können einen `Moderator` für einen Benutzer erstellen, der zu unserem Tenant gehört, um seine Moderationsstatistiken zu verfolgen:

[inline-code-attrs-start title = 'Moderator Erstellen via Tenant-Benutzer cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "userId": "some-tenant-user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Moderator Erstellen Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator Erstellen Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** Included on failure. **/
    reason?: string
    moderator?: Moderator; // We return the complete created moderator on success.
}
[inline-code-end]
