אם אינך יכול להתקין את [Shopify App Store app](https://apps.shopify.com/fastcomments), עדיין ניתן להוסיף את FastComments על‑ידי עריכת התבנית (theme) שלך. נתיב זה שימושי כאשר ברצונך לחבר Tenant של FastComments שבבעלותך, או כאשר אתה משלב את המערכת בחנות Shopify שבה האפליקציה אינה אפשרות.

התקנה באמצעות האפליקציה היא הנתיב הנתמך לרוב החנויות. השתמש בדרך זו רק אם האפליקציה אינה מתאימה.

### Step 1: Disable Shopify's native comments

בלוח הניהול של Shopify, עבור אל **Blog posts > Manage blogs**, פתח כל בלוג והגדר **Comments are disabled** בפאנל הימני. שמור.

זה מונע הצגת מערכת התגובות המובנית של Shopify לצד FastComments.

### Step 2: Open the blog theme template

בממשק הניהול של Shopify:

1. Go to **Online Store > Themes**.
2. Under your current theme, click **Actions > Edit code**.
3. In the file browser on the left, open **Sections** and click `main-article.liquid`.

This is the template Shopify renders for a single blog article.

### Step 3: Paste the FastComments snippet

Scroll to roughly line 100 of `main-article.liquid`, just after the closing `</div>` of the article body. Paste the following snippet:

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

Replace `"demo"` with your own Tenant ID from [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Click **Save**.

### Step 4: Authorize your shop domain

Open a blog post on your live store. If you see an authorization error instead of the comment widget, FastComments needs to know your store is allowed to use this tenant. See [Domain Errors](/guide-installation-shopify.html#shopify-domain-errors).

### Adding FastComments to other pages

The same snippet works on any Liquid template, including product pages, custom pages, and the home page. Paste it where you want comments to appear and adjust `urlId` if you want a stable identifier per page (for example, `urlId: "{{ product.id }}"` on a product template).