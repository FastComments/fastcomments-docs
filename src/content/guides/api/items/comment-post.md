[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 10; api-resource-header-end]

This API endpoint provides the ability to create comments.

Common use cases are custom UIs, integrations, or imports.

Notes:

- This API will automatically create user objects in our system.
- Trying to save two comments with different emails, but the same username, will result in an error for the second comment. 
- If you are specifying `parentId`, and a child comment has `notificationSentForParent` as false, **we will send notifications for the parent comment**. This is done every hour (we batch the notifications together to decrease the number of emails sent).
- This API can be "live" if desired.
- Comments created via this API will show up in the Analytics and Moderation pages of the admin app.
- "bad words" are still masked in the commenter names and comment text if the setting is turned on.
- Comments created via this API can still be checked for spam if desired.

[inline-code-attrs-start title = 'Comment POST cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true&doSpamCheck=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"commenterEmail": "fordperfect@spaceship.com",
	"comment": "some comment",
	"votes": 1,
	"verified": true,
	"reviewed": true,
	"notificationSentForParent": true,
	"notificationSentForParentTenant": true,
	"avatarSrc": "https://static.fastcomments.com/1605337537848-DSC_0841.JPG",
	"isSpam": false,
	"approved": true,
	"locale": "en_us",
	"date": 1622644382148
}'
[inline-code-end]

[inline-code-attrs-start title = 'Comment POST Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Note that urlId is not here - it is in the request body.
interface CommentPostQueryParams {
    tenantId: string;
    API_KEY: string;
    doSpamCheck?: 'true' | 'false';
    isLive?: 'true' | 'false';
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment POST Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed';
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment';  
    /** Included on failure. **/
    reason?: string;
    /** The created comment. **/
    comment?: Comment;
    /** The associated user, which may or may not have already existed. **/
    user?: User;
}
[inline-code-end]
