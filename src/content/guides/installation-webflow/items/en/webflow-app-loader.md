Webflow's Designer can place an element on the canvas, but it can't run the code that turns that element into a live widget. A small FastComments loader script does that. You add it to your site once, and it powers every FastComments widget you insert - now and later.

### Add the loader

1. In the FastComments panel, enter your **Tenant ID** and choose your **Region** (US or EU). The panel builds your loader script.
2. Click **Copy loader script**.
3. In Webflow, open **Site Settings > Custom Code**, paste the script into the **Footer Code** box, and save.

The script looks like this:

[inline-code-start]
<script src="https://cdn.fastcomments.com/js/webflow-loader.min.js" data-fc-tenant-id="YOUR_TENANT_ID"></script>
[inline-code-end]

For the EU region, the panel points the script at `cdn-eu.fastcomments.com` and adds a `data-fc-region="eu"` attribute.

Site-wide Custom Code takes effect only on the published site, and it requires a paid Webflow Site plan.
