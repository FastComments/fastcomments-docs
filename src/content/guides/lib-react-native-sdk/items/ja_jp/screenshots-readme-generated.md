#### スキン: Erebus
![スキン: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### スキン: デフォルト
![スキン: デフォルト](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### ネイティブWYSIWYGエディター（画像サポート付き）!
![ネイティブWYSIWYGエディター（画像サポート付き）](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### リッチテキストエディター

このライブラリはリッチテキスト編集機能のために10tapエディタを使用しており、強力なWYSIWYG編集体験を提供します。

### 設定オプション

このライブラリは、Web実装と同様に、[fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts)に定義されているすべての設定オプションをサポートすることを目指しています。

### FastCommentsの概念

開始するにあたって知っておくべき主な概念は `tenantId` と `urlId` です。 `tenantId` はあなたの FastComments.com アカウント識別子です。 `urlId` はコメントスレッドが紐付けられる場所です。
これはページのURL、製品ID、記事IDなどが該当します。

### ユーザー通知

FastCommentsは[多くのシナリオ](https://docs.fastcomments.com/guide-notifications.html)に対する通知をサポートしています。通知は設定可能で、グローバルまたは通知／コメント単位でオプトアウトでき、ページレベルの購読をサポートするため、ユーザーは特定のページや記事のスレッドを購読できます。

たとえば、Secure SSO を使用してユーザーを認証し、その後未読通知を定期的にポーリングしてユーザーへプッシュすることが可能です。

未読のユーザー通知を取得して翻訳する方法については、[例: AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx)を参照してください。

### GIFブラウザ

デフォルトでは画像やGIFの選択は有効になっていません。画像やGIFのアップロードをサポートする方法については [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) を参照してください。ライブラリには検索と提供される画像を匿名化するGifブラウザが含まれており、単にそれを利用するだけです。

### パフォーマンス

パフォーマンスの問題を特定した場合は、再現用の例（使用したデバイスを含む）を添えてチケットを開いてください。パフォーマンスはすべての FastComments ライブラリにおいて最重要事項です。