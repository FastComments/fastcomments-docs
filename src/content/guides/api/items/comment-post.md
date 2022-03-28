[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 10; api-resource-header-end]

This API endpoint provides the ability to create comments.

Common use cases are custom UIs, integrations, or imports.

Notes:

- This API can update the comment widget "live" if desired.
- This API will automatically create user objects in our system if email is provided.
- Trying to save two comments with different emails, but the same username, will result in an error for the second comment. 
- If you are specifying `parentId`, and a child comment has `notificationSentForParent` as false, **we will send notifications for the parent comment**. This is done every hour (we batch the notifications together to decrease the number of emails sent).
- If you want to send welcome emails when creating users, or comment verification emails, set `sendEmails` to `true` in query parameters.
- Comments created via this API will show up in the Analytics and Moderation pages of the admin app.
- "bad words" are still masked in the commenter names and comment text if the setting is turned on.
- Comments created via this API can still be checked for spam if desired.
- Configuration such as max comment length, if configured via the Customization Rule admin page, will apply here.

The minimum data required to submit is a comment, that will show in the comment widget, is as follows:

[inline-code-attrs-start title = 'Minimum Comment POST cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"date": 1622644382148,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true
}'
[inline-code-end]

A more realistic request may look like:

[inline-code-attrs-start title = 'Comment POST cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true&doSpamCheck=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"avatarSrc": "https://static.fastcomments.com/1605337537848-DSC_0841.JPG",
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"commenterEmail": "fordperfect@spaceship.com",
	"date": 1622644382148,
	"isSpam": false,
	"locale": "en_us",
	"notificationSentForParent": true,
	"notificationSentForParentTenant": true,
	"reviewed": true,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true,
	"votes": 1,
	"votesUp": 2,
	"votesDown": 1
}'
[inline-code-end]

[inline-code-attrs-start title = 'Comment POST Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Whether the comment should appear "live" to users viewing instances of the comment widget with the same urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment POST Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment'
    /** Included on failure. **/
    reason?: string
    /** The created comment. **/
    comment?: Comment
    /** The associated user, which may or may not have already existed. **/
    user?: User
}
[inline-code-end]
