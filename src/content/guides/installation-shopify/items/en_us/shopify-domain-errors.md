If you installed FastComments from the Shopify App Store, your shop domain is added to the tenant's authorized domains automatically and you should not see a domain error. This page applies if you went through the manual install path, or if your storefront is served on a custom domain that wasn't registered with Shopify at the time the app installed.

You may get an authorization error that looks like this:

<div class="screenshot white-bg">
    <div class="title">Domain Configuration Missing</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-webflow-step-5.png" alt="Domain Configuration Missing" />
</div>

This means FastComments doesn't recognize the domain the widget is loading on as authorized for your tenant.

To fix it, add the domain to your FastComments account: [Configure Domains](https://fastcomments.com/auth/my-account/configure-domains).
