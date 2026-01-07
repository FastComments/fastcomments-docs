You may find that our Analytics metrics show slightly different numbers than, say, Google Ads © or similar products.

For sites with one comment widget per-page, the numbers provided by FastComments Analytics are very accurate, and if incorrect will be **lower** than the actual value, but not more.

If you have an SPA you may find the FastComments Analytics numbers are higher than those reported by your marketing products. This is because
the marketing product may only be tracking when the page is not loaded, but not every time a user does something in the page that might
trigger a new comment thread from showing, which would count as a page load to FastComments Analytics.

#### Technical Information

FastComments Analytics tracks every page load, and does not rely on randomness as an optimization. Every page load results on an in-memory count
being updated in each thread on each server, which is then periodically persisted to the database via an atomic operation.
