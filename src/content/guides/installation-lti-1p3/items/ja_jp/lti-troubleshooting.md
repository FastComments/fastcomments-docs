#### "登録トークンが見つからない、期限切れ、または既に使用されています"

登録用の URL に含まれるトークンは 30 分間有効で、1 回しか使用できません。LMS の処理にそれ以上時間がかかった場合、または登録が成功した後に再試行された場合、そのトークンは拒否されます。FastComments LTI 1.3 設定ページで新しい URL を生成して、やり直してください。

#### "Platform rejected registration"

お使いの LMS が登録ハンドシェイクを拒否しました。最も一般的な原因:

- **同じクライアント名ですでにツールが登録されている。** 一部のプラットフォーム（特に D2L）は、以前の登録が削除されるまで "FastComments" の二重登録を拒否します。LMS 内の古いツールを削除してから、再試行してください。
- **LMS のフィールドを間違えている。** URL をペーストするのは **registration / tool initiation registration endpoint** フィールドで、起動用 URL やログイン用 URL のフィールドではないことを確認してください。
- **LMS が実際には Dynamic Registration をサポートしていない。** 古いバージョンの Moodle や Blackboard は LTI 1.3 を謳っていますが、手動設定のみを許可する場合があります。プラットフォームのドキュメントを確認してください。

#### "Failed to fetch platform configuration"

FastComments は LMS の openid-configuration ドキュメントを読み取れませんでした。これは稀で、通常は LMS が不正なまたは到達不能な discovery URL を提供していることを意味します。LMS のサポートに連絡してください。

#### Launch shows "Configuration not found"

FastComments の設定が削除されたか、または起動が当社で認識していない `iss`/`client_id` の組み合わせから来ています。設定を削除して再登録した場合は、LMS に FastComments のツールを削除して再追加するよう指示し、新しい client_id を取得させてください。

#### Launch shows "Deployment not registered"

FastComments を最初に起動したものとは異なる Brightspace/Moodle/Blackboard のデプロイメントから起動しました。FastComments は初回起動時にセキュリティチェックとして `deployment_id` を固定します。同じクライアントの下に新しいデプロイメントを追加するにはサポートに連絡してください — 当社でその deployment ID を設定に追加します。

#### Launch shows "Unsupported message_type"

LMS が FastComments が処理しない LTI メッセージを送信しました（例: `LtiSubmissionReviewRequest`）。FastComments は標準の resource-link launch と deep-linking flows のみをサポートしています。特定のメッセージタイプを追加する必要がある場合はご連絡ください。

#### Iframe doesn't resize

ほとんどの LMS は LTI iframe の自動サイズ調整を行います。お使いの LMS が行わない場合は、LMS の起動設定がツールから親フレームへの postMessage イベントの送信を許可しているか確認してください。FastComments は Canvas スタイルの (`lti.frameResize`) と IMS 仕様の (`org.imsglobal.lti.frameResize`) の両方のリサイズメッセージを送出します。