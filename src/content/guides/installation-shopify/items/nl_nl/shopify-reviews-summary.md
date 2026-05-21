The **FastComments - Reviews Summary** block shows an aggregated star rating and review breakdown for a page. Pair it with the **FastComments** block on product templates for the standard reviews layout: summary up top, review form and reviews below.

### Vereiste: stel Ratings & Reviews in

The Reviews Summary block displays the rating questions you configured for your store. Set those up first:

1. Open the FastComments app in your Shopify admin.
2. Click the **Ratings & Reviews Helper** tile (or open [Ratings & Reviews Helper](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify) directly).
3. Add the questions you want each reviewer to answer (overall star rating, "how was the fit", etc.).

Without questions configured, the summary block has nothing to aggregate.

### Blok toevoegen

1. Open the Shopify theme editor.
2. Open the **Product** template (or the page template where you want the summary).
3. Click **Add block** near the top of the page section, above where the **FastComments** block will be.
4. Under **Apps**, select **FastComments - Reviews Summary**.
5. Add a **FastComments** block lower on the same page if you haven't already, so visitors can leave reviews.
6. Click **Save**.

### Instellingen

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Override which FastComments tenant the summary reads from. Leave blank to use the store's automatically-configured tenant. | (leeg) |
| Custom URL ID | Override the page identifier the summary aggregates against. Use this when the summary lives on a different page from the FastComments block it reflects. | (automatisch gedetecteerd) |

### Hoe de samenvatting overeenkomt met de reviews

The Reviews Summary block uses the same auto-detection logic as the **FastComments** block:

- Product template: `shopify-product-{product.id}`
- Blog post template: `shopify-article-{article.id}`
- Other templates: the request path

For a normal product page, the summary and the comment thread share a URL ID automatically, with no configuration needed.

### Tips

- The summary is read-only. To collect reviews, you need a **FastComments** block on the same page.
- If you change rating questions in Ratings & Reviews Helper after collecting reviews, the summary recalculates against the new question set.