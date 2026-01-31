Next we're going to scroll down to line `100`:

<div class="screenshot white-bg">
    <div class="title">גלול לשורה 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="גלול לשורה 100" />
</div>

Now copy the following code snippet, which is designed **specifically for Shopify - do not use code snippets from other tutorials**:

[inline-code-attrs-start title = 'קטע FastComments ל-Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        urlId: window.location.pathname
    }];
</script>
[inline-code-end]

Now we want to put our cursor on `line 101` - right after the `</div>` - and paste. You should have something like this:

<div class="screenshot white-bg">
    <div class="title">הוסף את קוד FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="הוסף את קוד FastComments" />
</div>

Now we can save:

<div class="screenshot white-bg">
    <div class="title">שמור</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="שמור" />
</div>

---