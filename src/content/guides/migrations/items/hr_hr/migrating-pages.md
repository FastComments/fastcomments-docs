---
While its name is the Domain Migration tool, it can also be used to migrate comments across individual pages.

Simply enter the old and new [urlId](/guide-customizations-and-configuration.html#url-id) values in the `from` and `to` input fields.

### Migrating Pages via API

To migrate many pages at once, use [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post). It takes up to 100 `from` and `to` pairs per request and runs them as a batch job.

With `type` set to `urlId`, only the page whose `urlId` is exactly `from` is moved, so `post-1` does not match `post-10`. The dashboard tool does not have this option.

Each API migration can be undone for 90 days with [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post).

---