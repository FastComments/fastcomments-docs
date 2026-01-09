For SSO there is the following configuration to consider for notifications:

- ユーザーが通知にオプトインしているかどうか。
  - これは `SSOUser` オブジェクトの `optedInNotifications` フラグを `true` または `false` に設定することで行います。
  - これは API 経由で設定できます。
  - また、ペイロードでこのフラグに値を渡すと、ユーザーがコメントスレッドを読み込んだときに自動的に更新されます。
- ユーザーが **subscription** 通知にオプトインしているかどうか。
  - これは `SSOUser` オブジェクトの `optedInSubscriptionNotifications` フラグを `true` または `false` に設定することで行います。
  - これは API 経由で設定できます。
  - また、ペイロードでこのフラグに値を渡すと、ユーザーがコメントスレッドを読み込んだときに自動的に更新されます。
- メールアドレスを定義すること。
  - メールアドレスが存在しない場合、メールベースの通知を送信できません。
- メール内の配信停止リンクを無効にするかどうか。
  - これは `Tenant` オブジェクトの `disableUnsubscribeLinks` フラグで行います。
  - これは API 経由で設定できます。
- カスタムの配信停止リンクを使用するかどうか。
  - これは `DomainConfig` オブジェクトの `footerUnsubscribeURL` プロパティで行います。
  - これは API 経由で設定できます。
  - 同じオブジェクトの `emailHeaders` を通じて、関連する配信停止ヘッダーを設定することも検討してください。