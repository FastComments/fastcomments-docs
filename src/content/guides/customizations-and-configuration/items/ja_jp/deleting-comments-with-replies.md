---
デフォルトでは、ユーザーは自分のコメントを削除できます。 また、コメントを削除すると自動的に
スレッド内のすべての子コメントおよび一時的なコメントも削除されます。この動作はライブ環境でも同様に適用されます。

これを以下の方法で制限できます：

- 代わりに、削除されたコメントを匿名化する（名前とテキストを `[deleted]` またはカスタム値に設定する）。
- 返信がある場合にコメントの削除を許可しない。カスタマイズ可能なエラーメッセージが表示されます。
- コメントに返信がある場合の削除を管理者とモデレーターのみに制限する。

これはウィジェットカスタマイズUIの `Comment Thread Deletion` セクションで設定できます。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; title='Customize Delete Behavior for Replies' app-screenshot-end]

---