Flexには固定の上限がないため、トラフィックの急増や暴走した連携によって、予定より大きな請求書が発生する可能性があります。支出上限を設定すると、請求の使用量に応じた部分に上限を設けることができます。

[Billing Info](https://fastcomments.com/auth/my-account/billing-info) ページのプラン概要には `Monthly spend limit` が表示されます。`Edit` をクリックしてダイアログを開きます。`Target monthly max` を入力してFastCommentsに各上限を自動的に合わせさせることができるほか、各上限を自分で設定することもできます:

- `Page loads per month`
- `Comments stored`
- `SSO users`
- `API credits per month`
- `Small widget credits per month`

フィールドを空のままにすると、その種類の使用量は無制限のままになります。ダイアログには、入力した上限に基づく `Estimated monthly max` が表示され、Flexの最低料金も引き続き適用されます。

### 上限の動作

上限に達すると、その種類の使用量は月末までブロックされます。ページ読み込みの場合、ウィジェットはコメントの読み込みを停止し、月が切り替わるか上限を引き上げるまで `This FastComments.com account has exceeded its limits.` と表示します。上限の引き上げは数分以内に反映されます。

支出上限は請求書全体に対する厳格な上限ではありません。上記の使用量の項目のみが対象です。モデレーターや追加ドメインなど、自分で追加したシートやサイトは、直接管理しているものであるため、上限に関係なく請求されます。次回の請求書のプレビューは、いつでも [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) で確認できます。

支出上限を変更できるのは `Account Owner` または `Billing Admin` のみです。