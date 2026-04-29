**テンプレートID:** `tos_enforcer`

Moderator テンプレートは、手動モデレーションの負荷を減らすことを目的とする場合に推奨される出発点です。新規およびフラグ付けされたコメントをレビューし、コミュニティのルールを適用します。

### 組み込み初期プロンプト

[inline-code-attrs-start title = 'モデレーターテンプレート初期プロンプト'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

ほとんどの場合、サイトで許可されるものと許可されないものの具体的な例でこのプロンプトを**拡張する**ことをお勧めします。プラットフォーム自身のエスカレーションポリシー（警告の後に禁止、禁止前にメモリ検索など）はエージェントが受け取るシステムプロンプトに既に組み込まれているため、繰り返す必要はありません。

### トリガー

- **新しいコメントが投稿された** (`COMMENT_ADD`) - エージェントはすべての新しいコメントを確認します。
- **コメントがフラグの閾値を超えた** (`COMMENT_FLAG_THRESHOLD`, デフォルト閾値: 3) - 他のユーザーがフラグを立てたコメントをエージェントが再評価します。

### 使用可能なツール

- [`mark_comment_approved`](#tools-overview) - エージェントがクリーンなコメントを公開し、残りを非表示にするプリモデレーションテナントで役立ちます。
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

コメントの投稿、投票、ピン留め、ロック、バッジ授与、メール送信はできません — プロンプトは意図的に機能を限定しています。

### 本番稼働前に推奨される追加項目

- **[コミュニティガイドライン](#community-guidelines)を設定する。** 数文の書かれたポリシーで十分です；エージェントは各実行時にそれを適用します。
- **`ban_user` を [承認](#approval-workflow) の背後に設置する。** これはEUリージョンではデフォルトでオンになっており（[EU DSA Article 17 Compliance](#eu-dsa-compliance) を参照）、どこでも推奨されます。
- **低量だが高リスクのコンテンツがある場合は `mark_comment_spam` の承認を検討してください。**
- **プリモデレーションを実行する場合は `mark_comment_approved` を承認の背後に設置してください。** 悪いコメントを承認すると読者の目に触れるため、エージェントがドライランで信頼を築くまで制限してください。
- [コンテキストオプション](#context-options)で「コメント投稿者の信頼度、アカウント年齢、禁止履歴、最近のコメントを含める」にチェックを入れてください。モデルは、ユーザーが長期間の善意ある利用者であることが分かると、はるかに慎重に警告を出します。

### 推奨ドライラン期間

このテンプレートを [dry-run](#dry-run-mode) で少なくとも1週間、実際のトラフィックに対して稼働させてから有効化してください。過去30日分に対するプレビューには [Test Runs (Replays)](#test-runs-replays) を使用してください。

---