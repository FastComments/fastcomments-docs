---
預設情況下，使用者可以刪除自己的評論。且刪除評論時會自動刪除該討論串中所有子評論和暫時性評論。此行為亦為即時生效。

您可以透過以下方式限制此行為：

- 改為匿名化已刪除的評論（將名稱與文字設為 `[deleted]` 或自訂值）。
- 當評論有回覆時不允許刪除。會顯示可自訂的錯誤訊息。
- 僅允許管理員與版主在評論有回覆時進行刪除。

此設定可於 Widget Customization UI 中的 `Comment Thread Deletion` 區段進行配置。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; alt='在小工具自訂 UI 中的 Comment Thread Deletion 選項，用於匿名化或限制帶有回覆的刪除'; title='自訂回覆的刪除行為' app-screenshot-end]

---