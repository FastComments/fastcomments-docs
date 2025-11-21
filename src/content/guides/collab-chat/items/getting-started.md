### Quick Start

Getting started with Collab Chat is simple. You need the FastComments Collab Chat script, an HTML element containing the text you want to annotate, and a configuration object with your Tenant ID.

### Installation

Add the Collab Chat script to your page:

[inline-code-attrs-start title = 'Loading the Collab Chat Script'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Basic Implementation

Here's a minimal example:

[inline-code-attrs-start title = 'Basic Collab Chat Implementation'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Your content container -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Load the Collab Chat script -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Initialize Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Replace `'demo'` with your actual FastComments Tenant ID if it's not already, which you can find in your [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

### How It Works

Once initialized, users can select any text within the target element. After a brief delay (3.5 seconds on desktop), a prompt appears allowing them to start a discussion. When a discussion is created, a visual highlight appears on the text. Other users can hover over or click the highlight to view and participate in the discussion. All discussions sync in real-time across all visitors.

### Live Demo

You can see Collab Chat in action on our [live demo page](https://fastcomments.com/product/collab-chat).

### Next Steps

Now that you have the basics working, you can customize the appearance and behavior in the Configuration Options guide. Check out the Text Selection Behavior guide to understand how text selection works. Learn about styling and dark mode support in the Customization guide. For advanced integrations, explore the API Reference.

### Frontend Libraries

All FastComments frontend libraries (react, vue, angular, etc) have Collab Chat.
