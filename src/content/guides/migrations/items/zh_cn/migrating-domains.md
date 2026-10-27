FastComments 为您提供了一种自动化方式，将评论跨域迁移。

域迁移只需提供一个 `from` 域和一个 `to` 域。

这会 **移动** 评论，而不是复制它们。如果您希望复制评论，请联系我们。

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='带有来源和目标域字段以及已迁移评论计数的域迁移工具'; title='迁移域' app-screenshot-end]

这在以下情况下也很有用，例如，如果您迁移到 FastComments 的一部分涉及从其他提供商迁移，那么您的评论导入数据可能包含需要迁移的数据。在这种情况下，您可以先运行导入，然后进行域迁移。

### 监控进度

域迁移工具使用与其他数据管理工具相同的 FastComments 作业处理系统。

在迁移开始之前可能会有延迟。这是正常的，因为系统会定期检查新作业以进行处理。

作业运行时，它会显示发现的待迁移评论数量以及迄今为止已迁移的数量。

### 通过 API 迁移域

也可以使用 [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post) 将域迁移，`type` 设置为 `domain`。进度可通过 [`GET /api/v1/batch-jobs/:id`](/guide-api.html#batch-job-get-by-id) 获取，迁移可在 90 天内撤销。

---