The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### The WebhookComment Object Structure

##### The "Create" Event Structure
The "create" event request body is a WebhookComment object.

##### The "Update" Event Structure
The "update" event request body is a WebhookComment object.

##### The "Delete" Event Structure
The "delete" event request body is a WebhookComment object, but only containing the id.

[inline-code-attrs-start title = 'The WebhookComment Object'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    id: string
    urlId: string
    url: string
    userId: string
    commenterEmail: string
    commenterName: string
    comment: string
    commentHTML: string
    parentId: string
    date: DateString
    votes: number
    verified: boolean
    verifiedDate: number
    reviewed: boolean
    avatarSrc: string
    isSpam: boolean
    aiDeterminedSpam: boolean
    hasImages: boolean
    pageNumber: number
    approved: boolean
    locale: string
}
[inline-code-end]

#### HTTP Methods Used

**Create and Update both use HTTP PUT and not POST!**

Since all of our requests contain an ID, repeating the same Create or Update request should not create new objects on your side.

This means that these calls are idempotent and should be PUT events as per the HTTP specification.
