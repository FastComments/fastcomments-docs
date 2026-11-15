チャージ(トップアップ)は、アカウントに保存されているお支払い方法に請求されます。まだカードを保存していない場合、クレジットページには `Add a payment method first` と表示され、`Go to billing` ボタンが表示されます。[Billing Info](https://fastcomments.com/auth/my-account/billing-info) ページでカードを保存してから、戻ってきてください。

クレジットを追加するには:

1. [My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info) を開き、`Manage Credits` をクリックします。
2. `Add credit` の下で、プリセットの金額のいずれかを選択するか、`Other amount` を選択して任意の金額を入力します。
3. `Add` ボタンをクリックします。選択した金額が表示されます。例: `Add $25.00`。
4. 銀行から支払いの確認を求められた場合は、開いたウィンドウ内で確認を完了してください。

支払いが完了すると、ページに `Credit added` と新しい残高が表示され、チャージは `Past top-ups` に `Credited` として表示されます。`Add more` をクリックすると、再度チャージできます。

### 金額と上限

プリセットは、請求通貨で `10`、`25`、`50`、`100` です。カスタム金額は、1回のチャージにつき `5` から `10,000` までの範囲で指定でき、お使いの通貨の正確な上限はフォームの下に表示されます。日本円など小数点以下の桁がない通貨の場合、プリセットと上限は100倍の値になり、ほぼ同じ金額を表します。

チャージは常にアカウントの請求通貨、つまり請求書と同じ通貨で行われます。どの通貨で請求されるかはページに表示されます。

### 銀行確認(3Dセキュア)

一部の銀行では、オンライン支払いの確認が必要です。通常は銀行アプリまたはワンタイムコードを使用します。お使いの銀行がこれに該当する場合、`Add` をクリックした後に確認ウィンドウが開きます。カードへの請求と残高の更新は、銀行が承認した後にのみ行われます。

確認を完了する前にページを閉じた場合、チャージは `Pending` として記録されます。次にクレジットページを開いたときに、Stripeで支払いのステータスを確認し、それに応じて `Credited` または `Failed` としてマークします。銀行が支払いを承認しなかった場合、ページには `Your bank did not approve the payment. Try again, or use a different card.` と表示され、請求は行われません。

### カードが拒否された場合

カードが拒否された場合、ページには `Your card was declined. Check your card on the billing page, or try another card.` と表示されます。請求は行われず、残高は変更されず、その試行は `Past top-ups` に `Failed` として記録されます。[Billing Info](https://fastcomments.com/auth/my-account/billing-info) ページでカードを更新し、再試行してください。

カードを保護するため、アカウントは1時間あたり限られた回数しかチャージを試行できません。`Too many top-up attempts. Wait a while and try again.` と表示された場合は、しばらく待ってから再試行してください。

支払いはStripeによって処理されます。FastCommentsがお客様のカード情報を保存することはありません。