---
The **FastComments - Reviews Summary** block shows an aggregated star rating and review breakdown for a page. Pair it with the **FastComments** block on product templates for the standard reviews layout: summary up top, review form and reviews below.

### Prerequisite: set up Ratings & Reviews

The Reviews Summary block displays the rating questions you configured for your store. Set those up first:

1. Open the FastComments app in your Shopify admin.
2. Click the **Ratings & Reviews Helper** tile (or open [Ratings & Reviews Helper](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify) directly).
3. Add the questions you want each reviewer to answer (overall star rating, "how was the fit", etc.).

Without questions configured, the summary block has nothing to aggregate.

### Add the block

1. Open the Shopify theme editor.
2. Open the **Product** template (or the page template where you want the summary).
3. Click **Add block** near the top of the page section, above where the **FastComments** block will be.
4. Under **Apps**, select **FastComments - Reviews Summary**.
5. Add a **FastComments** block lower on the same page if you haven't already, so visitors can leave reviews.
6. Click **Save**.

### Settings

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | מגדיר מאיזה tenant של FastComments הבלוק הסיכום קורא. השאר ריק כדי להשתמש ב-tenant שהוגדר אוטומטית לחנות. | (ריק) |
| Custom URL ID | מחליף את מזהה הדף שלפיו הבלוק הסיכום מבצע אגירה. השתמש בכך כאשר הסיכום נמצא בדף שונה מהבלוק FastComments שהוא משקף. | (זוהה אוטומטית) |

### How the summary matches the reviews

The Reviews Summary block uses the same auto-detection logic as the **FastComments** block:

- תבנית מוצר: `shopify-product-{product.id}`
- תבנית פוסט בבלוג: `shopify-article-{article.id}`
- תבניות אחרות: נתיב הבקשה

For a normal product page, the summary and the comment thread share a URL ID automatically, with no configuration needed.

### Tips

- הסיכום הוא לקריאה בלבד. כדי לאסוף חוות דעת, יש להציב בלוק **FastComments** באותו דף.
- אם תשנה את שאלות הדירוג ב-Ratings & Reviews Helper לאחר שאספת חוות דעת, הסיכום יחושב מחדש בהתאם לערכת השאלות החדשה.

---