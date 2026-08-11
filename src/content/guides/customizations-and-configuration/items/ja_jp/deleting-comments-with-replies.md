---
デフォルトでは、ユーザーは自分のコメントを削除できます。また、コメントを削除すると、スレッド内のすべての子コメントおよび一時的なコメントが自動的に削除されます。この動作はリアルタイムでも適用されます。

次の方法でこれを制限できます：

- 代わりに、削除されたコメントを匿名化します（名前とテキストを `[deleted]` またはカスタム値に設定）。
- 返信がある場合、コメントの削除を許可しません。カスタマイズ可能なエラーメッセージが表示されます。
- コメントに返信がある場合の削除を、管理者とモデレーターのみに制限します。

`Comment Thread Deletion` セクションで、ウィジェットカスタマイズ UI から設定できます。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; alt='ウィジェットカスタマイズ UI における、返信がある削除を匿名化または制限するためのコメントスレッド削除オプション'; title='返信に対する削除動作のカスタマイズ' app-screenshot-end]
---