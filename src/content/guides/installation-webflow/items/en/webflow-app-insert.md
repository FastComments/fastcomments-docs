### Insert a widget

1. In the Webflow Designer, click the spot on the canvas where the widget should appear. For a blog, open your **CMS Collection Template** page and click inside the container where comments should show - the widget will then appear on every post automatically.
2. In the FastComments panel, pick a **Widget**: Comments, Live Chat, Reviews Summary, Comment Count, Recent Comments, Recent Discussions, Top Pages, or User Activity.
3. Leave **Thread ID** on **Page URL path** (recommended). Each published page - including every CMS item - gets its own comment thread automatically, with nothing to configure. Choose **Custom ID** only when you want a fixed thread.
4. Click **Insert widget**.

The app drops a placeholder element onto the canvas. It stays empty in the Designer and in Preview - that's expected. The loader turns it into a live widget on the published site.

### Keep threads stable across URL changes

By default the thread is tied to the page path, so renaming a CMS item's slug starts a fresh thread. To keep the same thread across slug changes, bind a `data-fc-url-id` custom attribute on the inserted element to your Collection item's ID field, using Webflow's element settings. Set it once on the Collection Template and every post keeps a stable thread.
