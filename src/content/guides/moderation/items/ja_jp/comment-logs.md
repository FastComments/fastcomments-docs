FastComments は、モデレーションの決定やシステムの動作の透明性を提供するために、各コメントに対して詳細なイベントを自動的に追跡します。これらのログは、コメントが承認された理由、スパムとしてフラグ付けされた理由、またはステータスが変更された理由を理解するのに役立ちます。

## コメントログへのアクセス

特定のコメントのログを表示するには:

1. FastComments ダッシュボードの **Moderate Comments** ページに移動します
2. 調べたいコメントを見つけます
3. コメントのアクションバーにある **View Logs** ボタン（時計アイコン）をクリックします
4. そのコメントのイベント履歴全体を表示するダイアログが表示されます

各ログエントリには次が表示されます:
- **When** - イベントのタイムスタンプ
- **Who** - イベントを引き起こしたユーザーまたはシステム（該当する場合）
- **What** - アクションやイベントの種類
- **Details** - 変更前/変更後の値、エンジン名、関連データなどの追加コンテキスト

## コメントログイベント

各コメントはライフサイクル中に発生するイベントのログを保持します。以下は追跡されるイベントの種類です:

### Anonymization Events
- **Anonymized** - コメント内容が消去され、ユーザーが削除済みとしてマークされた
- **RestoredFromAnonymized** - 匿名化状態からコメントが復元された

### Approval Events
- **ApprovedDueToPastComment** - ユーザーが以前に承認されたコメントを持っているためコメントが承認された（過去のコメントへの参照を含む）
- **ApprovedIsAdmin** - ユーザーが管理者であるためコメントが承認された
- **NotApprovedRequiresApproval** - コメントは手動承認が必要
- **NotApprovedLowTrustFactor** - 低いユーザートラストファクターのためコメントが承認されなかった（トラストファクターの値を含む）

### Profile Comment Approval Events

これらのイベントは、ユーザープロフィール上のコメントに特に適用されます:

- **ApprovedProfileAutoApproveAll** - プロフィール所有者がすべてのコメントを自動承認する設定にしているためプロフィールコメントが自動承認された
- **ApprovedProfileTrusted** - プロフィールコメントが信頼された投稿者のため承認された（信頼を確立したコメントへの参照を含む）
- **NotApprovedProfileManualApproveAll** - プロフィール所有者が手動承認を有効にしているためプロフィールコメントは手動承認が必要
- **NotApprovedProfileNotTrusted** - 投稿者が信頼されていないためプロフィールコメントは承認されなかった
- **NotApprovedProfileNewUser** - 投稿者が新規ユーザーであるためプロフィールコメントは承認されなかった

### Spam Detection Events
- **IsSpam** - 検出エンジンによってコメントがスパムとしてフラグ付けされた（どのエンジンが判定したかを含む）
- **IsSpamDueToBadWords** - 卑語フィルタによりコメントがスパムとしてフラグ付けされた
- **IsSpamFromLLM** - AI/LLM エンジンによりコメントがスパムとしてフラグ付けされた（エンジン名、応答、トークン数を含む）
- **IsSpamRepeatComment** - 繰り返しのためコメントがスパムとしてフラグ付けされた（どのエンジンが検出したかを含む）
- **NotSpamIsOnlyImage** - 画像のみを含むためコメントはスパムとしてフラグ付けされなかった
- **NotSpamIsOnlyReacts** - リアクションのみを含むためコメントはスパムとしてフラグ付けされなかった
- **NotSpamNoLinkOrMention** - 疑わしいリンクやメンションがないためコメントはスパムとしてフラグ付けされなかった
- **NotSpamPerfectTrustFactor** - 高いユーザートラストのためコメントはスパムとしてフラグ付けされなかった
- **NotSpamTooShort** - 分析するには短すぎるためコメントはスパムとしてフラグ付けされなかった
- **NotSpamSkipped** - スパムチェックがスキップされた
- **NotSpamFromEngine** - 検出エンジンによってスパムではないと判断された（エンジン名とトラストファクターを含む）

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - 卑語フィルタのチェックでエラーが発生した
- **BadWordsFoundBadPhrase** - 卑語フィルタが不適切なフレーズを検出した（検出されたフレーズを含む）
- **BadWordsFoundBadWord** - 卑語フィルタが不適切な単語を検出した（検出された単語を含む）
- **BadWordsNoDefinitionForLocale** - コメントの言語に対する卑語定義が存在しない（ロケールを含む）

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - コメントは承認に検証が必要だがユーザーは検証済みセッションにない
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - コメントは承認に検証が必要だがユーザーはまだ検証されていない
- **InVerifiedSession** - コメントを投稿しているユーザーは検証済みセッションにいる
- **SentVerificationEmailNoSession** - 検証メールが未検証ユーザーに送信された
- **SentWelcomeEmail** - 新規ユーザーにウェルカムメールが送信された

### Trust and Security Events
- **TrustFactorChanged** - ユーザーのトラストファクターが変更された（変更前と変更後の値を含む）
- **SpamFilterDisabledBecauseAdmin** - 管理者ユーザーのためスパムフィルタがバイパスされた
- **TenantSpamFilterDisabled** - テナント全体でスパムフィルタが無効化された
- **RepeatCommentCheckIgnored** - 繰り返しコメントチェックが無視された（理由を含む）
- **UserIsAdmin** - ユーザーが管理者であると識別された
- **UserIsAdminParentTenant** - ユーザーが親テナントの管理者であると識別された
- **UserIsAdminViaSSO** - SSO を介してユーザーが管理者であると識別された
- **UserIsMod** - ユーザーがモデレーターであると識別された

### Comment Status Changes

ステータス変更イベントには変更前と変更後の値、ならびに変更を行ったユーザーが含まれます:

- **ExpireStatusChanged** - コメントの有効期限ステータスが変更された
- **ReviewStatusChanged** - コメントのレビュー状態が変更された
- **SpamStatusChanged** - コメントのスパム状態が更新された
- **ApproveStatusChanged** - コメントの承認状態が変更された
- **TextChanged** - コメントのテキスト内容が編集された（変更前と変更後のテキストを含む）
- **VotesChanged** - コメントの投票数が更新された（詳細な投票内訳を含む）
- **Flagged** - コメントがユーザーによってフラグ付けされた
- **UnFlagged** - コメントのフラグが解除された

### Moderation Actions
- **Pinned** - モデレーターによってコメントがピン留めされた（誰がピン留めしたかを含む）
- **UnPinned** - モデレーターによってコメントのピン留めが解除された（誰が解除したかを含む）

### Notification Events
- **CreatedNotifications** - コメントに対して通知が作成された（通知数を含む）
- **NotificationCreateFailure** - 通知の作成に失敗した
- **BadgeAwarded** - コメントに対してユーザーバッジが授与された（バッジ名を含む）

### Publishing Events
- **PublishedLive** - コメントがライブ購読者に公開された（購読者数を含む）

### Integration Events
- **WebhookSynced** - コメントがウェブフック経由で同期された

### Spam Rule Events
- **SpamRuleMatch** - カスタムスパムルールにコメントがマッチした（ルールの詳細を含む）

### Localization Events
- **LocaleDetectedFromText** - コメントテキストから言語ロケールが自動検出された（検出された言語とロケールを含む）

## コメントログのユースケース

コメントログは自動的に生成され、各コメントに保存されます。これらは以下のような価値ある洞察を提供します:

- **モデレーション決定の理解** - コメントがなぜ承認されたのか、レビュー保留になったのか、スパムとしてマークされたのかを正確に確認する
- **承認/スパムの問題のデバッグ** - コメントが期待通りに動作しない場合に決定ロジックをトレースする
- **ユーザー行動パターンの追跡** - トラストファクターの変化や検証ステータスを監視する
- **モデレーター操作の監査** - 特定のコメントに対してモデレーターが行った操作を確認する
- **スパムフィルタの有効性の調査** - どの検出エンジンがスパムを検出しているか、どのエンジンが検出できていないかを確認する
- **統合のトラブルシューティング** - ウェブフックの同期や通知配信を検証する

これらのログはモデレーションプロセスの透明性を維持し、コメントシステムの動作を微調整するのに役立ちます。