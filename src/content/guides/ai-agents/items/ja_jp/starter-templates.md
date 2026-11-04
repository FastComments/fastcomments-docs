FastCommentsには5つのスターターテンプレートが用意されており、ゼロから動作するエージェントを書く必要はありません。テンプレートには、[AI Agentsページ](https://fastcomments.com/auth/my-account/ai-agents)から**Browse templates**をクリックしてアクセスできます。

テンプレートを選択すると:

1. エージェントは**Status: Dry Run**で作成され、テンプレートに基づいた内部名が付けられます(`tos_enforcer`、`welcome_greeter`、`top_comment_pinner`、`thread_summarizer`、`gaslight_detector`)。その名前がテナント上で既に使用されている場合は、数値の接尾辞が追加されます。
2. プロンプト、トリガー、許可されたアクション、しきい値など、すべてが事前入力された編集フォームに直接移動します。上部に「Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready.」というバナーが表示されます。
3. まだ何も有効化されていません。保存してドライランをオンのままにする(観察用)か、Enabledに切り替えるまで、エージェントは動作しません。

### 5つのテンプレート

- **[Moderator](#template-moderator)** - 新着コメントとフラグ付きコメントをレビューし、初回の違反者には警告を発し、警告後でのみBANにエスカレーションします。新着コメントとフラグしきい値の超過時(デフォルトのフラグしきい値: 3)にトリガーされます。許可されたツール: `mark_comment_approved`、`mark_comment_spam`、`warn_user`、`ban_user`。

- **[Welcome Greeter](#template-welcome-greeter)** - 初めてコメントするユーザーに、短く個人的な温かいウェルカム返信をします。新規ユーザーの初回コメント時にトリガーされます。許可されたツール: `write_comment`。

- **[Top Comment Pinner](#template-top-comment-pinner)** - 実質的なトップレベルコメントが投票しきい値(デフォルト: 10)を超えたときにピン留めし、その際に以前ピン留めされていたコメントをまず解除します。投票しきい値の超過時にトリガーされます。許可されたツール: `pin_comment`、`unpin_comment`。

- **[Thread Summarizer](#template-thread-summarizer)** - 長いスレッドに対して、遅延後に中立的な1段落の要約を投稿し、それをピン留めします。スレッドが落ち着いてから要約するため、30分の遅延を設けて新着コメント時にトリガーされます。許可されたツール: `write_comment`、`pin_comment`、`unpin_comment`。

- **[Gaslight Detector](#template-gaslight-detector)** - スレッド途中で返信を歪めるようなコメント編集を監視し、元のテキストを復元して、作者にDMを送ります。コメント編集時にトリガーされます。許可されたツール: `edit_comment`、`warn_user`、`send_dm`。

### テンプレートのカスタマイズ

テンプレートは出発点であり、契約ではありません。以下を行うことが想定されています:

- コミュニティの声に合わせて**Initial prompt**を調整する。
- エージェントの実行頻度に合わせて**Triggers**を追加または削除する。
- 機密性の高いアクションには**Approvals**を追加する - モデレータータイプのテンプレートでは、`ban_user`を承認で保護することを強くお勧めします。
- エージェントがあなたの書かれたポリシーを一貫して適用できるよう、**Community guidelines**を追加する。[Community Guidelines](#community-guidelines)を参照してください。
- 予想されるトリガー数に応じた、エージェントごとの**Budgets**を設定する。

テンプレートは、適切なデフォルト値を事前入力するための単なる手段です。保存後は、エージェントはあなたのものになります。