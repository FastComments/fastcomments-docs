---
#### スキン: Erebus
![スキン: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### スキン: Default
![スキン: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### ネイティブ WYSIWYG エディター（画像対応）!
![ネイティブ WYSIWYG エディター（画像対応）](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### リッチテキストエディター

このライブラリはリッチテキスト編集機能に10tapエディタを使用しており、強力なWYSIWYG編集体験を提供します。

### 設定オプション

このライブラリは、ウェブ実装と同様に、[fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) に定義されたすべての設定オプションをサポートすることを目指しています。

### FastComments の概念

開始するために知っておくべき主な概念は `tenantId` と `urlId` です。`tenantId` はあなたの FastComments.com アカウント識別子です。`urlId` はコメントスレッドが紐づく場所を示します。これはページのURL、製品ID、記事IDなどでありえます。

### ユーザー通知

FastComments は[多くのシナリオ](https://docs.fastcomments.com/guide-notifications.html)に対する通知をサポートします。通知は設定可能で、全体または通知／コメント単位でオプトアウトでき、ページ単位の購読をサポートしているため、ユーザーは特定のページや記事のスレッドを購読できます。

たとえば、Secure SSO を使用してユーザーを認証し、未読通知を定期的にポーリングしてユーザーへプッシュすることが可能です。

未読のユーザー通知を取得して翻訳する方法については、[the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) を参照してください。

### GIF ブラウザ

デフォルトでは、画像や GIF の選択は有効になっていません。画像および GIF のアップロードをサポートする方法については、[example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) を参照してください。このライブラリには検索や画像を匿名化する Gif ブラウザが用意されており、それを使用するだけで利用できます。

### パフォーマンス

パフォーマンスの問題を特定した場合は、再現手順の例と使用したデバイスを含めてチケットを開いてください。パフォーマンスはすべての FastComments ライブラリにおいて最優先事項です。
---