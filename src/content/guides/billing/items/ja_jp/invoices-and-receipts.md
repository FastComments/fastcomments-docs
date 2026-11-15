プランの更新、Flex月、プラン変更のいずれであっても、すべての課金には請求書が発行されます。請求書は支払い完了後、領収書としても機能します。

### 請求書の確認

[Billing History](https://fastcomments.com/auth/my-account/billing-history)を開くか、請求情報ページの`View Invoices`をクリックしてください。テーブルには、日付、請求期間、請求書番号、説明、合計、ステータス(`Paid`、`Unpaid`、`Draft`、または`Void`)、支払期日とともに各請求書が、新しいものから順に1ページ10件ずつ一覧表示されます。

各請求書について:

- `View`はStripeのホストページで請求書を開きます。そこからダウンロードもできます。
- `PDF`はPDFを直接ダウンロードします。
- `Pay Now`は未払いの請求書で`View`の代わりに表示され、その請求書のStripe支払いページへ移動し、そこで直接支払うことができます。

請求書は作成時に、請求先メールアドレスにも送信されます。アカウント所有者ではなく経理用メールボックスに送信したい場合は、[Invoice Settings](#invoice-settings)で`Billing Email`を設定してください。

### 請求書に記載される内容

請求書には、[Invoice Settings](#invoice-settings)の請求先名・住所とVAT / Tax ID、明細行、適用された前払い[クレジット](#credits)、カードに請求された金額が表示されます。Flexの請求書では、使用量の種類ごとに1行が記載されます。詳細は[How Flex Usage Is Measured](#how-flex-usage-is-measured)を参照してください。欧州連合内の請求先住所の場合、Flexの請求書はVAT 0%で発行され、フッターに`Reverse Charge Transaction: Customer to pay VAT`と表示されます。

請求書設定の変更は、保存後に作成される請求書に適用されます。すでに発行された請求書は編集できないため、次の請求日より前に会社情報とVAT / Tax IDを設定してください。

### 他で請求されるアカウント

請求が外部で処理されている場合(例：サポートとの取り決めによる請求書払い)、請求書はカードへの請求ではなくお客様への送付となり、Billing Historyに表示されない場合があります。ShopifyまたはBigCommerce経由でインストールされたアカウントは、そのプラットフォームによって請求され、請求情報ページからそちらのプランページへリンクしています。親アカウント経由で請求されるサブアカウントには、独自の請求書はありません。[When You Are Charged](#when-you-are-charged)を参照してください。