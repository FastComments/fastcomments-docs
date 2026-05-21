If you can't install the [Shopify App Store app](https://apps.shopify.com/fastcomments), you can still add FastComments by editing your theme. This path is useful when you want to wire up a FastComments tenant you already own, or when you're embedding on a Shopify storefront where the app isn't an option.

The app-based install is the supported path for most stores. Reach for this only if the app doesn't fit.

### Step 1: Disable Shopify's native comments

In your Shopify admin, go to **Blog posts > Manage blogs**, open each blog, and set **Comments are disabled** in the right-hand panel. Save.

This stops Shopify's built-in commenting from showing alongside FastComments.

### Step 2: Open the blog theme template

In your Shopify admin:

1. Go to **Online Store > Themes**.
2. Under your current theme, click **Actions > Edit code**.
3. In the file browser on the left, open **Sections** and click `main-article.liquid`.

This is the template Shopify renders for a single blog article.

### Step 3: Paste the FastComments snippet

Scroll to roughly line 100 of `main-article.liquid`, just after the closing `</div>` of the article body. Paste the following snippet:

[inline-code-attrs-start title = 'Фрагмент FastComments для Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Replace `"demo"` with your own Tenant ID from [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Click **Save**.

### Step 4: Authorize your shop domain

Open a blog post on your live store. If you see an authorization error instead of the comment widget, FastComments needs to know your store is allowed to use this tenant. See [Domain Errors](/guide-installation-shopify.html#shopify-domain-errors).

### Adding FastComments to other pages

The same snippet works on any Liquid template, including product pages, custom pages, and the home page. Paste it where you want comments to appear and adjust `urlId` if you want a stable identifier per page (for example, `urlId: "{{ product.id }}"` on a product template).