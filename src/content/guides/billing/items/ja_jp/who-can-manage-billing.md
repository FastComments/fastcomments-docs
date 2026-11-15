請求は以下の2種類のユーザーに制限されています:

- `Account Owner`(アカウントを作成したユーザー)
- `Billing Admin` 権限を持つユーザー

これらのユーザーだけが、プランの変更、カードの保存や入れ替え、`Invoice Settings` の表示、Flex の利用限度額の設定、クレジットの追加を行うことができます。それ以外のユーザーが試みると、`Only the account owner or a billing admin can change the plan or payment method.` というメッセージが表示されます。

請求に関するメール(トライアル中のリマインダー、支払い失敗の警告、アカウント停止のお知らせなど)は、`Billing Admin` 権限を持つユーザーに送信されます。例えば経理担当者など、これらのメールを受け取るべき人に権限を付与すれば、ダッシュボードへのその他のアクセス権がなくてもメールを受け取ることができます。トライアルが請求情報の登録なしに終了した後に送られるリマインダーは、アカウントのすべてのダッシュボードユーザーに送られます。これらのメールにはそれぞれ配信解除リンクが付いており、アカウント通知をオプトアウトしたユーザーは受信しなくなります。

請求書自体は [Invoice Settings](#invoice-settings) の `Billing Email` に送られます。このメールアドレスは、ダッシュボードユーザーのものとは限りません。

[Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) ページは、支払い情報ではなく利用状況を表示するため、代わりに `Analytics Admin` 権限が必要です。

権限を付与または削除するには、`Users` ページを開いてユーザーを編集してください。[Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions) を参照してください。Flex では、追加の管理者ごとに課金対象のシートが発生することに注意してください。[Admin and Moderator Seats](#admin-and-moderator-seats) を参照してください。

ログインしているのに `Billing Info` がまったく表示されない場合や、ホームページに戻されてしまう場合は、ダッシュボードユーザーではなくコメント投稿者アカウントとしてログインしています。サインアウトして、サイトを所有するアカウントでサインインしてください。