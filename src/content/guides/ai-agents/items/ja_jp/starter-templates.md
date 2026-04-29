FastComments は、ゼロから動作するエージェントを書く必要がないように、4つのスターターテンプレートを提供しています。これらは [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents) の **Browse templates** をクリックすると利用できます。

テンプレートを選ぶと:

1. エージェントは **Status: Dry Run** で作成され、テンプレートに基づく内部名が付与されます（`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`）。その名前があなたのテナントですでに使われている場合は、数値のサフィックスが追加されます。
2. 編集フォームに直接移動し、プロンプト、トリガー、許可されたアクション、およびしきい値などがすべて事前入力されています。上部のバナーには "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready." と表示されます。
3. まだ何も有効になっていません。エージェントは保存して dry-run のまま（観察用）にしておくか、Enabled に切り替えるまで動作しません。

### The four templates

- **[Moderator](#template-moderator)** - 新規およびフラグ付きコメントをレビューし、初回の違反者に警告を行い、警告の後でのみ禁止へエスカレーションします。Triggers on new comments and on flag-threshold crossings (default flag threshold: 3). Allowed tools: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - 初めてコメントするユーザーに対して、短く親しみやかな個別の歓迎メッセージで応答します。Triggers on new-user-first-comment. Allowed tool: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - 重要なトップレベルコメントが投票しきい値を超えたとき（デフォルト: 10）、それをピン留めし、まず以前にピン留めされていたコメントのピンを外します。Triggers on vote-threshold crossings. Allowed tools: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - 長いスレッドに対して遅延後に中立的な一段落の要約を投稿し、それをピン留めします。スレッドが落ち着くように要約は 30 分の猶予を置いてから行われます。Triggers on new comments with a 30-minute deferral so the thread settles before summarizing. Allowed tools: `write_comment`, `pin_comment`, `unpin_comment`.

### Customizing a template

テンプレートは出発点であり、契約ではありません。次のことが期待されます:

- コミュニティの声に合わせて **Initial prompt** を調整してください。
- エージェントが実行される頻度に合わせて **Triggers** を追加または削除してください。
- 敏感なアクションには **Approvals** を追加してください — モデレーター向けテンプレートでは `ban_user` を承認制にすることを強く推奨します。
- エージェントがあなたの文書化された方針を一貫して適用するように **Community guidelines** を追加してください。参照: [Community Guidelines](#community-guidelines)。
- 想定されるトリガー数に応じて、エージェントごとの **Budgets** を設定してください。

テンプレートは妥当なデフォルトを事前入力するためのものであり、保存すればエージェントはあなたのものになります。