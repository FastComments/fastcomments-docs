「カード編集」用の個別画面はありません。登録済みのカードを差し替えるには、[Billing Info](https://fastcomments.com/auth/my-account/billing-info) を開き、プランはそのままにして、`Payment method` に新しいカードの情報を入力し、`Save Info` をクリックしてください。

新しいカードは、今後のすべての請求書のデフォルトになります。保存しても課金は発生しません。ただし、未払いの請求書がある場合は、その請求書が新しいカードで回収されます。固定プランでは即時、Flex では翌日中に処理されます。請求状態が正常な場合、フォームには `You won't be charged now unless you change plans.` と表示されます。固定プランのサブスクリプションが終了している場合、カードを保存すると新しいサブスクリプションが開始され、初回登録時と同様に最初の期間の料金が請求されます。

銀行がカードの確認を求める場合、保存をクリックした後に確認ウィンドウが開きます。カードは銀行が承認した後にのみ保存されます。

### この操作ができるユーザー

`Account Owner` または `Billing Admin` 権限を持つユーザーのみが、カードの保存やプランの変更を行えます。その他のユーザーには `Only the account owner or a billing admin can change the plan or payment method.` と表示されます。`Settings` メニューがグレーアウトしている場合や、請求ページからホームページにリダイレクトされる場合は、アカウントオーナーではなく、コメント投稿者または請求権限のないユーザーとしてログインしています。サイトを作成したアカウントでログインするか、オーナーに権限の付与を依頼してください。詳細は [Who Can Manage Billing](#who-can-manage-billing) を参照してください。

### カードが拒否またはブロックされた場合

カードが拒否された場合、ページには `Your card was declined. Check that the entered information is correct.` と表示され、何も保存されません。カード番号、有効期限、CVC、請求先住所を確認してから、再試行してください。銀行が確認を拒否した場合は、`Your bank did not approve the payment. Try again, or use a different card.` と表示されます。

一部の銀行はオンラインでの継続課金をブロックしたり、一度の拒否後にカードをブロックしたりします。その場合は別のカードを試すか、[Billing History](https://fastcomments.com/auth/my-account/billing-history) の `Pay Now` で未払いの請求書を支払ってください。銀行が自動課金を一切許可しない場合は、[サポートに連絡](https://fastcomments.com/auth/my-account/help) いただければ、請求書の支払いリンクをお送りできます。

利用できるのはカードのみです。フォームの `Country` リストには、Stripe が請求可能な国が含まれています。該当する国が見つからない場合は、サポートにお問い合わせください。