Now let's add our widget code.

Copy the code below. You'll want to ensure you're signed in to [fastcomments.com](https://fastcomments.com) 
and reload this page if you are not, so the code is pre-populated with your account information, otherwise it will show the demo code.

Now let's copy the code:

[inline-code-attrs-start title = 'Zyro Comments Code'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    });
</script>
[inline-code-end]

Now let's go back to our site builder and click `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Enter code</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Enter Code" />
</div>

### Note!

It's important that you use the above code and not the code snippets from other documentation, because this snippet has been crafted specifically
for Zyro.

You should now have something like the following, which appears blank. This is expected. Move your mouse over the area
where the widget should be:

<div class="screenshot white-bg">
    <div class="title">Code Widget Added</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Code Widget Added" />
</div>

Now drag the widget to the desired size — you'll see it appear:

<div class="screenshot white-bg">
    <div class="title">Resize It</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Resize It" />
</div>

...and now preview and save!

---