### Overview

FastComments Image Chat extends the standard FastComments commenting widget, so it inherits all the configuration options from the base widget while adding a few specific to image annotations.

### Required Configuration

#### tenantId

Your FastComments Tenant ID is required. You can find this in your [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat Specific Options

#### urlId

By default, Image Chat generates a unique identifier for each conversation based on the page URL, the image source, and the X/Y coordinates. You can override this with a custom `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

This is useful when your URL structure might change but you want to keep the same conversations, or when you want to share annotations across multiple pages.

#### chatSquarePercentage

Controls the size of the clickable chat markers as a percentage of the image width. The default is 5%, meaning each marker is 5% of the image width.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Larger, more visible markers
});
```

Smaller values create less intrusive markers that work better for detailed images. Larger values make markers easier to see and click on busy images or for users on mobile devices.

#### hasDarkBackground

Enable dark mode styling when your page has a dark background.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

A callback function that fires whenever the comment count changes. This is useful for updating UI elements like badges or page titles.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Inherited Configuration Options

Since Image Chat extends the standard commenting widget, you can use any configuration option from the base FastComments widget. Here are some commonly used options:

#### locale

Set the language for the widget UI. FastComments supports dozens of languages.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
```

#### readonly

Make all conversations read-only. Users can view existing markers and discussions but cannot create new ones or reply.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

Integrate with your authentication system using Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // SSO configuration
    }
});
```

See the SSO documentation for full details on authentication options.

#### maxReplyDepth

Control how many levels deep replies can go. By default, Image Chat sets this to 0, meaning all comments are flat (no nested replies). You can change this if you want threaded conversations.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Allow 3 levels of nesting
});
```

### Internal Configuration

These options are automatically set by Image Chat and should not be overridden:

The `productId` is automatically set to `2` for Image Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly with fullscreen chat windows.

### Target Element Flexibility

The first parameter to `FastCommentsImageChat` can be either an `<img>` element directly or a container element with an image inside:

```javascript
// Direct image element
FastCommentsImageChat(document.getElementById('my-image'), config);

// Container with image inside
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

The widget will find the image automatically if you pass a container element.

### Complete Example

Here's an example showing multiple configuration options together:

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // Your SSO config
    },
    maxReplyDepth: 1
});
```

For a complete list of all available configuration options inherited from the base widget, see the main FastComments configuration documentation.
