#### "登録トークンが見つからない、期限切れ、または既に使用されています"

あなたの登録用URL（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ここで取得できます</a>）内のトークンは30分間有効で、1回しか使用できません。LMS の処理にそれ以上時間がかかった場合、または登録が成功した後に再試行された場合、トークンは拒否されます。FastComments の LTI 1.3 設定ページで新しい URL を生成して最初からやり直してください。

#### "プラットフォームが登録を拒否しました"

あなたの LMS が登録ハンドシェイクを拒否しました。最も一般的な原因:

- **同じクライアント名ですでにツールが登録されています。** 一部のプラットフォーム（特に D2L）は、前のものが削除されるまで「FastComments」の2回目の登録を拒否します。LMS で古いツールを削除してから再試行してください。
- **LMS の誤ったフィールドに入力しています。** URL を **registration / tool initiation registration endpoint** フィールドに貼り付けたことを確認してください。launch URL や login URL のフィールドではありません。
- **LMS が実際には Dynamic Registration をサポートしていません。** 古い Moodle や Blackboard のバージョンは LTI 1.3 を謳っていますが、手動設定のみを許可します。プラットフォームのドキュメントを確認してください。

#### "プラットフォーム構成の取得に失敗しました"

FastComments があなたの LMS の openid-configuration ドキュメントを読み取れませんでした。これは稀で、通常は LMS が不正なまたは到達不能な discovery URL を提供したことを意味します。LMS サポートに連絡してください。

#### 起動時に "設定が見つかりません" と表示される

FastComments の設定が削除されたか、起動が当社で認識していない `iss`/`client_id` の組み合わせから行われました。設定を削除して再登録した場合は、LMS に FastComments ツールを削除して再追加するよう指示し、新しい client_id を取得するようにしてください。

#### 起動時に "デプロイメントが登録されていません" と表示される

最初に起動したものとは異なる Brightspace/Moodle/Blackboard のデプロイメントから FastComments を起動しました。FastComments はセキュリティチェックとして初回起動時に `deployment_id` を固定します。同じクライアントの下で新しいデプロイメントを追加するにはサポートに連絡してください — 我々が設定に deployment ID を追加します。

#### 起動時に "message_type がサポートされていません" と表示される

LMS が FastComments が処理しない LTI メッセージを送信しました（例: `LtiSubmissionReviewRequest`）。FastComments は標準の resource-link launch と deep-linking flows のみをサポートします。特定の message type を追加してほしい場合はお問い合わせください。

#### iframe がリサイズされない

ほとんどの LMS は LTI iframe を自動でサイズ調整します。あなたの LMS がそうでない場合は、LMS の起動設定がツールから親フレームへの postMessage イベント送信を許可しているか確認してください。FastComments は Canvas スタイル（`lti.frameResize`）と IMS 仕様（`org.imsglobal.lti.frameResize`）の両方のリサイズメッセージを送出します。