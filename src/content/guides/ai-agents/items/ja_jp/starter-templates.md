FastCommentsは、最初から動作するエージェントをゼロから書く必要がないように、5つのスターターテンプレートを提供しています。これらは[AI Agentsページ](https://fastcomments.com/auth/my-account/ai-agents)の**テンプレートを参照**をクリックすると表示できます。

When you pick a template:

1. エージェントは **ステータス：ドライラン** で作成され、テンプレートに基づく内部名が付与されます（`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`）。その名前がテナント内で既に使用されている場合は、数字のサフィックスが追加されます。
2. すべてが事前入力された編集フォームに直接移動します — プロンプト、トリガー、許可されたアクション、および任意の閾値。上部にはバナーが表示され、「{templateName} テンプレートから作成されました。以下の設定を確認し、準備ができたらステータスを有効に変更してください。」と表示されます。
3. まだ何も有効になっていません。エージェントは保存し、ドライランをオンのまま（観察用）にするか、有効に切り替えるまで動作しません。

### 5つのテンプレート

- **[Moderator](#template-moderator)** - 新規および通報されたコメントをレビューし、初回の違反者には警告を行い、警告後にのみ禁止にエスカレートします。トリガーは新しいコメントおよび flag-threshold の超過時（デフォルト flag threshold: 3）。許可されるツール: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - 初めてコメントするユーザーに対し、短くパーソナルな歓迎メッセージで温かく返信します。トリガー: new-user-first-comment。許可されるツール: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - 実質的な最上位コメントが vote threshold を超えるとピン留めし、まず以前ピンされていたコメントのピンを外します。トリガー: vote-threshold の超過時。許可されるツール: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - 長いスレッドに対して遅延の後に中立的な一段落の要約を投稿し、それをピン留めします。トリガーは新しいコメントで、要約の前にスレッドが落ち着くよう30分の遅延が入ります。許可されるツール: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - スレッド内の返信を歪めるような途中での書き換えを監視し、元のテキストを復元して投稿者にDMを送ります。トリガー: comment edits。許可されるツール: `edit_comment`, `warn_user`, `send_dm`.

### テンプレートのカスタマイズ

テンプレートは出発点であり、固定の契約ではありません。次のことが期待されます:

- コミュニティの声に合わせて **Initial prompt** を調整してください。
- エージェントの実行頻度に合わせて **Triggers** を追加または削除してください。
- 敏感なアクションには **Approvals** を追加してください — モデレーター向けテンプレートでは `ban_user` を承認で制限することを強く推奨します。
- エージェントが書面化されたポリシーを一貫して適用するよう **Community guidelines** を追加してください。詳細は[コミュニティガイドライン](#community-guidelines)を参照してください。
- 想定されるトリガー数に応じて各エージェントの **Budgets** を設定してください。

テンプレートは妥当なデフォルトを事前入力するための出発点に過ぎません；保存すれば、そのエージェントはあなたのものです。