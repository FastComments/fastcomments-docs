To make the Weebly and FastComments integration work nicely, we have to add **two** small pieces of code.

The first snippet is to hide the Weebly "Comments are Closed" message, and the second is to actually load FastComments.

First, copy this small code snippet:

[inline-code-attrs-start title = 'FastComments Header Code Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<style>
    #commentArea:not(.loaded) {
        display: none;
    }
</style>
[inline-code-end]

Then, on the same settings page from `Step One`, click the `+` next to `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Open Post Header Code</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Open Post Header Code" />
</div>

You should see a text box open like this:

<div class="screenshot white-bg">
    <div class="title">Post Header Code Open</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Post Header Code Open" />
</div>

Now let's paste in our code snippet:

<div class="screenshot white-bg">
    <div class="title">Header Code Snippet Pasted</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Header Code Snippet Pasted" />
</div>

Next up is the footer code to enable FastComments. Click the plus sign next to `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Open Post Footer Code</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Open Post Footer Code" />
</div>

Copy this code snippet which is designed **specifically for Weebly**:

[inline-code-attrs-start title = 'FastComments Footer Code Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        let interval = null;
        function attemptLoad() {
            if (loaded) {
                clearInterval(interval);
                return;
            }
            const commentArea = document.getElementById('commentArea');
            if (!commentArea) {
                return;
            }
            commentArea.innerHTML = '';
            commentArea.classList.add('loaded');
            FastCommentsUI(commentArea, {
                tenantId: "demo",
                urlId: window.location.pathname
            });
            loaded = true;
            clearInterval(interval);
        }
        attemptLoad();
        interval = setInterval(attemptLoad, 300);
    })();
</script>
[inline-code-end]

Now let's paste in our footer code:

<div class="screenshot white-bg">
    <div class="title">Post Footer Code Added</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Post Footer Code Added" />
</div>

That's it!
