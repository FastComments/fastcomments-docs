#### スキン: Erebus
![スキン: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### スキン: Default
![スキン: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### 画像サポート付きネイティブWYSIWYGエディタ!
![Native WYSIWYG Editor with Image Support](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### リッチテキストエディタ

このライブラリはリッチテキスト編集のために[`react-native-enriched`](https://github.com/software-mansion/react-native-enriched)を使用しており、強力なWYSIWYG編集体験を提供します。同じエディタがiOS、Android、およびウェブ（`react-native-web`経由）を支えているため、作成者は単一の実装であらゆるプラットフォーム上で一貫した動作をします。

`react-native-enriched`はネイティブ側でReact Native New Architecture (Fabric)を必要とし、パッケージの`exports`条件を解決するバンドラ（package exports対応のMetro / RN 0.72+）が必要です。ウェブサポートは現在実験的です。

### 構成オプション

このライブラリは、ウェブ実装と同様に[fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts)で定義されているすべての構成オプションをサポートすることを目指しています。

### FastComments の概念

開始するために把握しておくべき主な概念は`tenantId`と`urlId`です。`tenantId`はあなたのFastComments.comアカウント識別子です。`urlId`はコメントスレッドが紐付けられる場所です。これはページのURL、商品ID、記事IDなどになり得ます。

### ユーザー通知

FastCommentsは[多くのシナリオ](https://docs.fastcomments.com/guide-notifications.html)に対する通知をサポートしています。通知は設定可能で、グローバルまたは通知／コメント単位でオプトアウトでき、ページレベルの購読もサポートしているため、ユーザーは特定のページや記事のスレッドを購読できます。

例えば、Secure SSOを使ってユーザーを認証し、その後未読の通知を定期的にポーリングしてユーザーにプッシュすることが可能です。

未読のユーザー通知を取得して翻訳する方法については、[the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx)を参照してください。

### Gif ブラウザ

デフォルトでは、画像やgifの選択は有効になっていません。画像やgifのアップロードをサポートする方法については[example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx)を参照してください。本ライブラリには検索と提供される画像を匿名化するGifブラウザがあり、単にそれを使用するだけです。

### パフォーマンス

パフォーマンスの問題を特定した場合は、再現例（使用したデバイスを含む）を添えてチケットを開いてください。パフォーマンスはすべてのFastCommentsライブラリにおける第一級の重要事項です。