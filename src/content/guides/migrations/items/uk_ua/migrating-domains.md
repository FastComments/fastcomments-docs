FastComments надає автоматизований спосіб перенесення ваших коментарів між доменами.

The domain migration simply requires a `from` and a `to` domain.

This **moves** comments, it does not copy them. If you wish to copy comments, reach out to us.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='Інструмент міграції доменів з полями from і to та кількістю перенесених коментарів'; title='Міграція доменів' app-screenshot-end]

This is also useful for example if part of your migration to FastComments involves migrating from a different provider, so your comment import
data may contain data that needs to be migrated. In this case, you can run the import, and then the domain migration.

### Моніторинг прогресу

The Domain Migration tool uses the same FastComments job processing system as the other data management tools.

There may be a delay before your migration starts. This is normal, as it the system checks for new jobs to process periodically.

As the job runs, it will display the number of comments found to migrate, and the number migrated thus far.

### Міграція доменів через API

Domains can also be migrated with [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post), using `type` set to `domain`. Progress is available from [`GET /api/v1/batch-jobs/:id`](/guide-api.html#batch-job-get-by-id), and the migration can be undone for 90 days.

---