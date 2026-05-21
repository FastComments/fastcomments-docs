### Install from the Shopify App Store

1. Open the [FastComments listing on the Shopify App Store](https://apps.shopify.com/fastcomments).
2. Click **Add app** and pick the plan you want during the install flow.
3. Shopify redirects you back into the FastComments admin inside Shopify when the install completes.

That's the full install. There is nothing to paste into your theme files.

### What gets set up for you

The install runs everything you would otherwise do by hand:

- A FastComments tenant is created for your store and linked to your shop domain.
- Your shop's store URL is added to the tenant's authorized domains, so comments load without a domain error.
- A `fastcomments.tenant_id` shop metafield is written so every block knows which tenant to render against.
- Single sign-on for your Shopify customers is enabled by default.
- Billing runs through Shopify Managed Pricing. Charges appear on your regular Shopify bill. Upgrade, downgrade, or cancel from **Settings > Apps and sales channels > FastComments** in your Shopify admin.

If your shop was already a FastComments customer before you installed the app, the install reuses the existing tenant instead of creating a new one.

### The embedded admin

When you open the FastComments app from your Shopify admin, you land on a dashboard with one-click tiles into the full FastComments backend:

- **Dashboard**: account settings, usage, and subscription details.
- **Moderation Queue**: approve, reject, and reply to comments across your store.
- **Customize**: adjust widget colors, fonts, moderation rules, and configuration.
- **Ratings & Reviews Helper**: set up star ratings and review questions if you want to use the Reviews Summary block.

Each tile opens FastComments with a single-use login link, so you don't need a separate sign-in.

### Next: add blocks to your store

Open your Shopify theme editor (**Online Store > Themes > Customize**), open the template you want to add commenting or reviews to, and click **Add block**. The FastComments blocks appear under **Apps**. The rest of this guide covers each one.
