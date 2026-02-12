---
name: fastcomments
description: >
  FastComments is a real-time commenting platform with widget libraries for HTML/VanillaJS, React, Vue, Angular, Svelte,
  and React Native. It provides a full REST API, Secure SSO (HMAC-SHA256), extensive widget customization, comment
  moderation, webhooks, backend SDKs for 10+ languages, white-labeling via Tenant Packages, and migration tools for
  Disqus, Commento, WordPress, and more.
---

# FastComments Agent Skill

## Overview

FastComments is a fast, feature-rich commenting platform for websites and applications. It offers drop-in comment widgets, a REST API, SSO, moderation tools, webhooks, and white-label support.

- **Documentation site**: https://docs.fastcomments.com
- **TypeScript type definitions**: https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts
- **GitHub organization**: https://github.com/FastComments

## Documentation Search API

For detailed lookups on any FastComments topic, query the documentation search API:

```
GET https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo
```

- Replace `<search_query>` with URL-encoded search terms (e.g., `react`, `sso`, `webhooks`, `dark+mode`)
- Returns matching documentation sections with title, URL, and full body text
- Use this to find specific configuration options, API details, or integration guides

Example:
```bash
curl "https://docs-search.fastcomments.com/search?query=dark+mode&full=true&tenantId=demo"
```

## Quick Start - Widget Installation

### VanillaJS (HTML)

Add this snippet to any HTML page:

```html
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "TENANT_ID"
});
</script>
```

The widget automatically creates a separate comment thread per page URL. No account is needed for testing - use `tenantId: "demo"`.

### React

```bash
npm install fastcomments-react
```

```tsx
import { FastCommentsCommentWidget } from 'fastcomments-react';

function App() {
    return <FastCommentsCommentWidget tenantId="TENANT_ID" />;
}
```

For EU region: `<FastCommentsCommentWidget tenantId="TENANT_ID" region="eu" />`

### Vue 3

```bash
npm install fastcomments-vue-next
```

```vue
<template>
    <fast-comments v-bind:config="{ tenantId: 'TENANT_ID' }" />
</template>

<script>
import { FastComments } from 'fastcomments-vue-next';
export default {
    components: { FastComments }
};
</script>
```

Vue 2 is also supported via the `fastcomments-vue` package.

### Angular

```bash
npm install fastcomments-typescript ngx-fastcomments
```

Add to your module:

```typescript
import { FastCommentsModule } from 'ngx-fastcomments';

@NgModule({
    imports: [BrowserModule, FastCommentsModule],
    // ...
})
export class AppModule {}
```

Usage in templates:

```html
<lib-fastcomments [config]="{ tenantId: 'TENANT_ID' }"></lib-fastcomments>
```

### Svelte

```bash
npm install fastcomments-svelte
```

See the library on GitHub: https://github.com/FastComments/fastcomments-svelte

Available widgets: CommentWidget, StreamingChatWidget, CollabChatWidget, ImageChatWidget, CommentCountWidget, UserActivityFeedWidget.

### React Native

```bash
npm install fastcomments-react-native
```

```tsx
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

const config = {
    tenantId: 'TENANT_ID',
    urlId: 'page-or-article-id'
};

return <FastCommentsCommentWidget config={config} />;
```

## Key Concepts

- **tenantId**: Your account identifier. Use `"demo"` for testing. Find yours at https://fastcomments.com/auth/my-account/api-secret
- **urlId**: Identifies which page/article a comment thread belongs to. Defaults to the current page URL if not specified. Can be any string or URL.
- **region**: Set to `"eu"` if your account is in the EU region. Omit for US/default.
- Comments are automatically grouped by `urlId` - each unique value creates a separate thread.

## REST API

### Base URL and Authentication

```
https://fastcomments.com/api/v1/<resource>?tenantId=TENANT_ID&API_KEY=YOUR_API_KEY
```

For EU region accounts:
```
https://eu.fastcomments.com/api/v1/<resource>?tenantId=TENANT_ID&API_KEY=YOUR_API_KEY
```

Authentication is via query parameters: `tenantId` and `API_KEY`. Get your API key at https://fastcomments.com/auth/my-account/api-secret

### API Resources

| Resource | Endpoints | Description |
|----------|-----------|-------------|
| Comments | GET, POST, PATCH, DELETE | Create, read, update, delete comments |
| Comment Actions | POST block/unblock/flag/unflag | Moderate individual comments |
| Pages | GET, POST, PATCH, DELETE | Manage page metadata and URL IDs |
| SSO Users | GET, POST, PUT, PATCH, DELETE | Manage SSO user accounts |
| Moderators | GET, POST, PATCH, DELETE | Manage moderator accounts |
| Subscriptions | GET, POST, DELETE | Manage page subscriptions |
| Notifications | GET, PATCH | Read and update notifications |
| Notification Counts | GET, DELETE | Notification count management |
| Hash Tags | GET, POST, PATCH, DELETE | Manage hashtags |
| Email Templates | GET, POST, PATCH, DELETE | Custom email templates |
| Domain Configs | GET, POST, PUT, PATCH, DELETE | Domain-level configuration |
| Question Configs | GET, POST, PATCH, DELETE | Polling/question configuration |
| Question Results | GET, POST, PATCH, DELETE | Poll/question responses |
| Tenants | GET, POST, DELETE | Manage tenants (white-label) |
| Tenant Packages | GET, POST, PUT, PATCH, DELETE | White-label tenant packages |
| Tenant Daily Usage | GET | Usage analytics |
| Pending Webhook Events | GET, DELETE | Monitor webhook delivery |
| User Badges | POST | Assign badges to users |
| Audit Logs | GET | Account audit trail |
| Aggregate | POST | Aggregate operations |

### Comment Interface

```typescript
interface Comment {
    id: string                    // Unique comment ID (readonly)
    tenantId: string              // Tenant the comment belongs to
    urlId: string                 // Page/article identifier
    comment: string               // Raw comment text (markdown)
    commentHTML?: string          // Rendered HTML (readonly)
    commenterName: string         // Display name (required)
    commenterEmail?: string       // Email (required if anon commenting off)
    date: number                  // UTC epoch timestamp
    parentId?: string | null      // Parent comment ID for replies
    approved?: boolean            // Approval status
    verified: boolean             // User verification status
    reviewed: boolean             // Moderation review status
    url: string                   // Page URL
    avatarSrc?: string            // User avatar URL
    userId?: string | null        // User ID
    votes?: number                // Net votes (up - down)
    votesUp?: number              // Upvote count
    votesDown?: number            // Downvote count
    isPinned?: boolean            // Pinned status
    isDeleted?: boolean           // Soft-deleted flag
    isSpam?: boolean              // Spam flag
    isLocked?: boolean            // Locked for replies
    isByAdmin?: boolean           // Posted by admin (readonly)
    isByModerator?: boolean       // Posted by moderator (readonly)
    displayLabel?: string         // Label shown next to name
    locale?: string               // Comment locale
    meta?: Record<string, string | number | boolean>  // Custom metadata
    hashTags?: CommentHashTag[]   // Parsed hashtags
    mentions?: CommentUserMention[]  // Parsed @mentions
    children: Comment[]           // Child comments (when asTree=true)
}
```

Full details: https://docs.fastcomments.com/guide-api.html

## SSO (Single Sign-On)

FastComments Secure SSO uses HMAC-SHA256 encryption. No new API endpoints are needed - encrypt the user payload server-side and pass it to the widget.

### SSO Flow

1. User visits your page
2. Your server creates a JSON payload with user info, Base64-encodes it
3. Your server creates an HMAC-SHA256 hash of `TIMESTAMP + userDataJSONBase64` using your API secret
4. Pass `sso` object to the widget config with `userDataJSONBase64`, `verificationHash`, `timestamp`, `loginURL`, `logoutURL`
5. Widget authenticates the user automatically

### SSOUser Interface (Widget Payload)

```typescript
interface SSOUser {
    id: string;                          // Required, unique user ID
    email: string;                       // Required, unique email
    username: string;                    // Required, cannot be an email
    avatar?: string;                     // URL or base64 image
    optedInNotifications?: boolean;      // Email notifications
    optedInSubscriptionNotifications?: boolean; // Subscription emails
    displayLabel?: string;               // Label next to name (e.g., "VIP")
    displayName?: string;                // Shown instead of username
    websiteUrl?: string;                 // Links user name
    groupIds?: string[];                 // Access control groups
    isAdmin?: boolean;                   // Administrator flag
    isModerator?: boolean;               // Moderator flag
    isProfileActivityPrivate?: boolean;  // Default true
    isProfileCommentsPrivate?: boolean;  // Default false
    isProfileDMDisabled?: boolean;       // Default false
}
```

### Code Examples

Full working SSO examples in Node.js, Java/Spring, and PHP: https://github.com/FastComments/fastcomments-code-examples/tree/master/sso

## Common Widget Configuration

Top configuration options (all optional except `tenantId`):

| Option | Type | Description |
|--------|------|-------------|
| `tenantId` | string | **Required.** Your account ID |
| `urlId` | string | Page identifier for the comment thread |
| `region` | string | Set to `"eu"` for EU accounts |
| `readonly` | boolean | Disable new comments |
| `hasDarkBackground` | boolean | Dark mode styling |
| `locale` | string | UI language (e.g., `"de_de"`, `"fr_fr"`) |
| `allowAnon` | boolean | Allow anonymous comments |
| `defaultSortDirection` | `"MR"` \| `"NF"` \| `"OF"` | Most Relevant, Newest First, Oldest First |
| `maxReplyDepth` | number | Maximum nesting depth for replies |
| `pageSize` | number | Comments per page |
| `customCSS` | string | URL to custom CSS file |
| `headerHTML` | string | Custom HTML above the widget |
| `noImageUploads` | boolean | Disable image uploads |
| `useSingleLineCommentInput` | boolean | Single-line input mode |
| `collapseReplies` | boolean | Collapse reply threads by default |
| `sso` | FastCommentsSSO | SSO configuration object |

### Callbacks

```javascript
{
    onInit: function() {},
    onRender: function() {},
    onAuthenticationChange: function(eventName, userObj) {},
    commentCountUpdated: function(count) {},
    onReplySuccess: function(comment) {},
    onVoteSuccess: function(comment, voteId, direction, status) {},
    onImageClicked: function(src) {},
    onOpenProfile: function(userId) {},
    onCommentSubmitStart: function(comment, continueFn, cancelFn) {},
    onCommentsRendered: function(comments) {}
}
```

Full type definitions: https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts

## Backend SDKs

| Language | Package | Install |
|----------|---------|---------|
| JavaScript/TypeScript | [fastcomments-sdk-js](https://github.com/FastComments/fastcomments-sdk-js) | `npm install fastcomments-sdk-js` |
| Python | [fastcomments-python](https://github.com/FastComments/fastcomments-python) | `pip install fastcomments` |
| Java | [fastcomments-java](https://github.com/FastComments/fastcomments-java) | Maven/Gradle |
| PHP | [fastcomments-php](https://github.com/FastComments/fastcomments-php) | `composer require fastcomments/fastcomments` |
| Ruby | [fastcomments-ruby](https://github.com/FastComments/fastcomments-ruby) | `gem install fastcomments` |
| Go | [fastcomments-go](https://github.com/FastComments/fastcomments-go) | `go get github.com/FastComments/fastcomments-go` |
| Rust | [fastcomments-rust](https://github.com/FastComments/fastcomments-rust) | Cargo |
| C++ | [fastcomments-cpp](https://github.com/FastComments/fastcomments-cpp) | CMake |
| Swift | [fastcomments-swift](https://github.com/FastComments/fastcomments-swift) | Swift Package Manager |
| Nim | [fastcomments-nim](https://github.com/FastComments/fastcomments-nim) | Nimble |
| PHP SSO | [fastcomments-php-sso](https://github.com/FastComments/fastcomments-php-sso) | `composer require fastcomments/fastcomments-sso` |

All SDKs are generated from the same OpenAPI spec and provide typed access to every API endpoint.

## Webhooks

FastComments supports webhooks for Comment events:

- **Comment Created** - Fired when a new comment is posted
- **Comment Updated** - Fired when a comment is edited
- **Comment Deleted** - Fired when a comment is removed

The initial webhook event is usually sent within six seconds. Failed deliveries are retried on a schedule of `1 minute * retry count` (exponential backoff).

Configure webhooks at: https://fastcomments.com/auth/my-account/manage-data/webhooks

Monitor and cancel pending events at: https://fastcomments.com/auth/my-account/manage-data/webhooks/logs

## White-Labeling

FastComments supports full white-labeling via the **Tenants** and **Tenant Packages** APIs.

- **Tenants API**: Create and manage sub-accounts (child tenants)
- **Tenant Packages API**: Define feature packages that can be applied to tenants, controlling what features and limits each tenant has

API endpoints: `GET/POST/PUT/PATCH/DELETE /api/v1/tenants` and `/api/v1/tenant-packages`

## Platform Installations

FastComments has dedicated installation guides for these platforms:

WordPress, Shopify, WooCommerce, Squarespace, Wix, Webflow, Ghost, Weebly, Blogger (via VanillaJS), BigCommerce, Bubble.io, ClickFunnels, Framer, GoDaddy, GoHighLevel, Hostinger, IONOS, MemberSpace, MemberStack, Moodle, Super.so (Notion), Systeme.io, ThriveCart Learn, Typeflo, Webnode, Zyro.

Each guide is at: `https://docs.fastcomments.com/guide-installation-<platform>.html`

## Migrations

FastComments natively supports importing data from:

- Disqus
- Commento
- WordPress (via plugin)
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments

Import page: https://fastcomments.com/auth/my-account/manage-data/import

Re-importing the same content will not create duplicates. Import files are deleted after processing.
