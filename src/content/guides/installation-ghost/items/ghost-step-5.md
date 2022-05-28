Next we need to identify where to add the FastComments.com widget code.

If you're using the default `casper` theme, you'll see a section like this at line `82`:

<div class="screenshot white-bg">
    <div class="title">Disabled Comment Section</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Disabled Comment Section" />
</div>

If you're using other themes, you won't see this, and will need to add this code after the last `</section>`:

[inline-code-attrs-start title = 'Godaddy Comments Code Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

You should have something like this ready:

<div class="screenshot white-bg">
    <div class="title">Template Ready For Comment Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Template Ready For Comment Code" />
</div>

Once ready, copy the FastComments.com widget code:

[inline-code-attrs-start title = 'Ghost FastComments.com Comment Code Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let simpleSSO = null;

        \{{#if access}}
            \{{#if @member}}
                simpleSSO = {
                    email: '\{{@member.email}}',
                    username: '\{{@member.firstname}}',
                    optedInNotifications: true,
                    displayLabel: '\{{@member.labels}}'
                }
            \{{/if}}
        \{{/if}}

        FastCommentsUI(document.getElementById('fastcomments-widget'), {
            tenantId: "demo",
            urlId: window.location.pathname,
            allowAnon: false,
            simpleSSO: simpleSSO
        });
    })();
</script>
[inline-code-end]

...and it should look like this:

<div class="screenshot white-bg">
    <div class="title">Add FastComments.com Comment Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Add FastComments.com Comment Code" />
</div>

Coding done. Now we just have to re-import our theme!
