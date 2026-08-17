This is the five-minute path from "we have AI Agents" to "an agent is responding to live traffic, gated by approvals." If you want the long form, every step links to the page that covers it in depth.

### 1. AI エージェントページを開く

アカウント内の [AI Agents](https://fastcomments.com/auth/my-account/ai-agents) に移動します。初めてこのページに来たときは、以下のいずれかが表示されます。

- エージェントを作成できる **Browse templates** と **Start from scratch** ボタンがある空白状態、または
- プランにエージェントが含まれていない場合のアップセルページ - [Plans and Eligibility](#plans-and-eligibility) を参照してください。

### 2. スターターテンプレートを選択

**Browse templates** をクリックします。以下から選択してください。

- [Moderator](#template-moderator) - フラグされたコメントや新規コメントをレビューし、初回投稿者に警告を出し、警告後にのみバンにエスカレーションします。
- [Welcome Greeter](#template-welcome-greeter) - 初めてコメントするユーザーに返信します。
- [Top Comment Pinner](#template-top-comment-pinner) - 投票閾値を超えた実質的なコメントをピン留めします。
- [Thread Summarizer](#template-thread-summarizer) - 長いスレッドに中立的な要約を投稿します。

各テンプレートは、すでに **Status: Dry Run** が選択された事前入力済みの編集フォームに表示されます。

### 3. 確認して保存

編集フォームで、最低限以下を行います。

- **Internal name.** 管理ダッシュボードで使用される短い識別子です。
- **Display name.** エージェントがコメントを投稿する際に公開される名前です。
- **Initial prompt.** テンプレートのプロンプトを編集して、あなたのトーンや特定のルールに合わせます。
- **Approvals.** 実行前に人間のレビューが必要なアクションにチェックを入れます。モデレーション系エージェントには少なくとも `ban_user` を推奨します。詳しくは [Approval Workflow](#approval-workflow) を参照してください。

**Save agent** をクリックします。

### 4. ドライランで様子を見る

エージェントは現在 **Dry Run** で稼働しています。トリガーを受け取り、モデルを呼び出し、[Run History](#run-history) ページにアクションを記録します（各行に **Dry Run** バッジが付く）— ただし実際のアクションは実行されません。いくつかの実行詳細（[Run Detail View](#run-detail-view) を参照）を確認し、次の点を見てください。

- エージェントが選択したアクション。
- 各アクションの根拠と信頼度。
- 完全な LLM トランスクリプト。

エージェントの判断に同意できない場合は、初期プロンプトを編集するか、承認項目にさらにチェックを入れてください。

### 5. 過去のコメントに対してテスト実行

エージェント一覧ページで、エージェントの行の **Test run** をクリックします。フォームには **Days** という数値入力（1〜90）が1つだけあります。サンプルサイズと評価対象コメント数の上限は情報として表示されますが、これはサーバー側で計算され、ユーザーが設定するものではありません。リプレイは実際のアクションを取らずに過去のコメントに対して実行され、エージェントが **実際に** 行ったであろう処理と、実際に起きたこと（コメントが後で承認されたか、スパムとしてマークされたか、削除されたかなど）を報告します。[Test Runs (Replays)](#test-runs-replays) を参照してください。

### 6. 有効化へ切り替え

ドライランとリプレイの結果に満足したら、エージェントを編集し **Status** を **Enabled** に変更します。これ以降は実際のアクションが実行されます。Run History ページはドライランバッジなしでライブ実行を表示し、承認が必要とマークしたアクションは [approvals inbox](#approval-workflow) に表示されます。

### 次のステップ

- [Budgets](#budgets-overview) と [Budget Alerts](#budget-alerts) を設定します。
- エージェントイベントに外部システムが反応するようにしたい場合は、[Webhooks](#webhooks-overview) を設定します。
- エージェントの判断が書面化されたポリシーと一致するように、[Community Guidelines](#community-guidelines) を追加します。

---