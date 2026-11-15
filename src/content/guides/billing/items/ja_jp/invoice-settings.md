[Invoice Settings](https://fastcomments.com/auth/my-account/invoice-settings) ページは、Billing Info の `Invoice Settings` ボタンからアクセスでき、請求書に表示される内容と送付先を制御します。開けるのは `Account Owner` または `Billing Admin` のみです。

- `Company Name`: ダッシュボード、メール、チーム招待に表示されるアカウント名です。請求先名ではありません。
- `Billing Email`: 請求書と請求関連の通知の送付先です。アカウントオーナーが受け取らないようにする場合は、財務・経理用のメールボックスを設定してください。ユーザーのログインメールアドレスは変更されません。
- `Invoice Currency`: 請求書が発行される通貨です。[Currencies](#currencies) を参照してください。
- `Billing Address`: 請求書に印字される `Name`、`Address`、`City`、`State / Province / Region`、`Postal Code`、`Country` です。カードを保存する際の Billing Info フォームにも同じ項目が表示され、両者は同じ場所に書き込まれます。
- `VAT / Tax ID`: 任意です。請求書に印字されます。

`Save Changes` をクリックしてください。ページに `Invoice settings updated successfully.` と表示され、変更は次回の請求書から適用されます。既存の請求書は変更されません。

### VAT And Tax IDs

ID を入力する前に、請求先の `Country` を選択してください。国によって ID の種類が決まるためです。EU 加盟国であれば EU VAT 番号、イギリスであれば UK VAT 番号、オーストラリアであれば ABN、カナダであれば GST/HST 番号、アメリカであれば EIN、その他 Stripe が対応している国についても同様です。

ID は保存時に検証されます。選択した国に対して有効でない場合、その旨がページに表示され、ID は保存されませんが、その他の設定は保存されます。税 ID を登録できない国の場合も、その旨がページに表示されます。

EU の請求先住所をお持ちの場合、Flex の請求書は逆課税取引として VAT 0% で発行され、保存された VAT 番号が請求書に表示されるため、VAT はお客様側の経理担当者が処理できます。