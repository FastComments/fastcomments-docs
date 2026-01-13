### Overview

FastComments Collab Chat extends the standard FastComments commenting widget, so it inherits all the configuration options from the base widget while adding a few specific to text annotations.

### Required Configuration

#### tenantId

Your FastComments Tenant ID is required. You can find this in your [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Configuration Example"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Collab Chat Specific Options

#### urlId

By default, Collab Chat generates a unique identifier for each conversation based on the page URL, the DOM path to the element, and the selected text range. You can override this with a custom `urlId`.

[inline-code-attrs-start title = "Configuration Example"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

This is useful when your URL structure might change but you want to keep the same conversations, or when you want to share annotations across multiple pages.

#### topBarTarget

Controls the display of the top bar which shows user count and discussion count. Set to `null` to disable the top bar entirely, or provide a DOM element to render it in a specific location.

[inline-code-attrs-start title = "Configuration Example"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Disable top bar
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Render top bar in custom location
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Enable dark mode styling when your page has a dark background. This detection is automatic, but it may be desirable to override it.

[inline-code-attrs-start title = "Configuration Example"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

A callback function that fires whenever the comment count changes. This is useful for updating UI elements like badges or page titles.

[inline-code-attrs-start title = "Configuration Example"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Inherited Configuration Options

Since Collab Chat extends the standard commenting widget, you can use any configuration option from the base FastComments widget. Here are some commonly used options:

#### locale

Set the language for the widget UI. FastComments supports dozens of languages.

[inline-code-attrs-start title = "Configuration Example"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
[inline-code-end]

#### readonly

Make all conversations read-only. Users can view existing annotations but cannot create new ones or reply.

[inline-code-attrs-start title = "Configuration Example"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Integrate with your authentication system using Single Sign-On.

[inline-code-attrs-start title = "Configuration Example"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO configuration
    }
});
[inline-code-end]

See the SSO documentation for full details on authentication options.

#### maxReplyDepth

Control how many levels deep replies can go. By default, Collab Chat sets this to 0, meaning all comments are flat (no nested replies). You can change this if you want threaded conversations.

[inline-code-attrs-start title = "Configuration Example"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Allow 3 levels of nesting
});
[inline-code-end]

### Internal Configuration

These options are automatically set by Collab Chat and should not be overridden:

The `productId` is automatically set to `3` for Collab Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly.

### Complete Example

Here's an example showing multiple configuration options together:

[inline-code-attrs-start title = "Configuration Example"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(document.getElementById('article'), {
    tenantId: 'demo',
    urlId: 'my-article-v2',
    hasDarkBackground: false,
    locale: 'en',
    topBarTarget: document.getElementById('custom-header'),
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) My Article` : 'My Article';
    },
    sso: {
        // Your SSO config
    },
    maxReplyDepth: 1
});
[inline-code-end]

For a complete list of all available configuration options inherited from the base widget, see the main FastComments configuration documentation.