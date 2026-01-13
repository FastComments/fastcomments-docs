預設情況下，使用者可以刪除自己的評論。此外，刪除評論會自動刪除該討論串中的所有子評論和暫存評論。此行為也會即時生效。

您可以透過以下方式限制此行為：

- 改為將被刪除的評論匿名化（將 name 和 text 設為 `[deleted]` 或自訂值）。
- 當有回覆時，不允許刪除評論。會顯示可自訂的錯誤訊息。
- 當評論有回覆時，僅允許管理員與版主刪除。

可在 Widget Customization UI 的 `Comment Thread Deletion` 區段進行設定。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; title='Customize Delete Behavior for Replies' app-screenshot-end]