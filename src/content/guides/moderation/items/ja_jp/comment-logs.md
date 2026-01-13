FastCommentsは各コメントの詳細なイベントを自動的に追跡し、モデレーションの判断やシステムの動作に関する透明性を提供します。これらのログは、コメントがなぜ承認されたのか、スパムとしてフラグが立てられたのか、またはステータスが変更されたのかを理解するのに役立ちます。

Moderate Comments ダッシュボードで特定のコメントを選択すると、個別のコメントログを表示できます。

## コメントログイベント

各コメントにはライフサイクル中に発生したイベントのログが保持されます。以下は追跡されるイベントの種類です：

### 匿名化イベント
- **Anonymized** - コメント内容が消去され、ユーザーが削除済みとしてマークされました

### 承認イベント
- **ApprovedDueToPastComment** - ユーザーが過去に承認されたコメントを持っているためコメントが承認されました
- **ApprovedIsAdmin** - ユーザーが管理者であるためコメントが承認されました
- **NotApprovedRequiresApproval** - コメントは手動承認が必要です

### スパム検出イベント
- **IsSpam** - 検出エンジンによってコメントがスパムとしてフラグ付けされました
- **IsSpamDueToBadWords** - 不適切な言葉フィルターによりコメントがスパムとしてフラグ付けされました
- **IsSpamFromLLM** - AI/LLMエンジンによってコメントがスパムとしてフラグ付けされました
- **IsSpamRepeatComment** - 繰り返しのためコメントがスパムとしてフラグ付けされました
- **NotSpamIsOnlyImage** - 画像のみを含むためコメントはスパムとしてフラグ付けされませんでした
- **NotSpamIsOnlyReacts** - リアクションのみを含むためコメントはスパムとしてフラグ付けされませんでした
- **NotSpamNoLinkOrMention** - 疑わしいリンクやメンションがないためコメントはスパムとしてフラグ付けされませんでした
- **NotSpamPerfectTrustFactor** - ユーザーの信頼度が高いためコメントはスパムとしてフラグ付けされませんでした
- **NotSpamTooShort** - 分析するには短すぎるためコメントはスパムとしてフラグ付けされませんでした
- **NotSpamSkipped** - スパムチェックがスキップされました
- **NotSpamFromEngine** - 検出エンジンによりスパムではないと判断されました

### 不適切語句/暴言イベント
- **BadWordsCheckFailed** - 不適切語句フィルターのチェック中にエラーが発生しました
- **BadWordsFoundBadPhrase** - 不適切なフレーズが検出されました
- **BadWordsFoundBadWord** - 不適切な単語が検出されました
- **BadWordsNoDefinitionForLocale** - コメント言語に対する不適切語句の定義が存在しません

### ユーザー検証イベント
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - コメントは検証が必要ですが、ユーザーは検証済みセッションにありません
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - コメントは検証が必要ですが、ユーザーはまだ検証されていません
- **InVerifiedSession** - コメントを投稿しているユーザーは検証済みセッションにあります
- **SentVerificationEmailNoSession** - 未検証ユーザーに検証メールが送信されました
- **SentWelcomeEmail** - 新規ユーザーにウェルカムメールが送信されました

### 信頼性とセキュリティイベント
- **TrustFactorChanged** - ユーザーの信頼度が変更されました
- **SpamFilterDisabledBecauseAdmin** - 管理者ユーザーのためスパムフィルターが回避されました
- **TenantSpamFilterDisabled** - テナント全体でスパムフィルターが無効化されました
- **RepeatCommentCheckIgnored** - 繰り返しコメントチェックが無視されました
- **UserIsAdmin** - ユーザーが管理者として識別されました
- **UserIsAdminParentTenant** - ユーザーが親テナントの管理者として識別されました
- **UserIsAdminViaSSO** - SSO経由でユーザーが管理者として識別されました
- **UserIsMod** - ユーザーがモデレーターとして識別されました

### コメントステータスの変更
- **ExpireStatusChanged** - コメントの有効期限ステータスが変更されました
- **ReviewStatusChanged** - コメントのレビュー状況が変更されました
- **SpamStatusChanged** - コメントのスパムステータスが更新されました
- **ApproveStatusChanged** - コメントの承認ステータスが変更されました
- **TextChanged** - コメントの本文が編集されました
- **VotesChanged** - コメントの投票数が更新されました
- **Flagged** - ユーザーによってコメントがフラグ付けされました
- **UnFlagged** - コメントのフラグが削除されました

### モデレーション操作
- **Pinned** - モデレーターによってコメントがピン留めされました
- **UnPinned** - モデレーターによってピン留めが解除されました
- **RestoredFromAnonymized** - コメントが匿名化状態から復元されました

### 通知イベント
- **CreatedNotifications** - コメントに対する通知が作成されました
- **NotificationCreateFailure** - 通知の作成に失敗しました
- **BadgeAwarded** - コメントに対してユーザーにバッジが授与されました

### 公開イベント
- **PublishedLive** - コメントがライブ購読者に公開されました

### 統合イベント
- **WebhookSynced** - コメントがWebhook経由で同期されました

### スパムルールイベント
- **SpamRuleMatch** - コメントがカスタムスパムルールに一致しました

## コメントログへのアクセス

コメントログは各コメントとともに自動的に生成され、保存されます。これらは以下のような貴重な洞察を提供します：

- モデレーションの判断を理解するため
- 承認やスパムの問題をデバッグするため
- ユーザーの行動パターンを追跡するため
- システムの操作を監査するため

これらのログはモデレーションプロセスの透明性を維持し、コメントシステムの動作を微調整するのに役立ちます。