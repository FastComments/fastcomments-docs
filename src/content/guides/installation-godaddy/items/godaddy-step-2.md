Now that you've added a custom HTML block, we can add the FastComments widget code.

**Use the following code for Godaddy, not code from other tutorials. This code is Godaddy-specific.**

Copy the following code:

[inline-code-attrs-start title = 'Godaddy Comments Code Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        if (top.location.pathname !== '/' && top.location.pathname.length > 1) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
        }
    })();
</script>
[inline-code-end]

This specific code snippet is designed to be compatible with Godaddy, and will also only show on your blog posts - not the homepage.

Now paste the code into the `Custom Code` area mentioned in `Step One`.

<div class="screenshot white-bg">
    <div class="title">Copy and Paste The Code</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Copy and Paste The Code" />
</div>

Hit Done in the top right:

<div class="screenshot white-bg">
    <div class="title">Click Done</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Click Done" />
</div>

That's it for step two!
