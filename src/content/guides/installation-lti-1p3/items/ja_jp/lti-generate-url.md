#### LTI 1.3 設定に移動

FastComments にサインインし、<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">LTI 1.3 設定ページ</a>に移動してください。

アカウントにまだ LTI アクセスがない場合は「LTI not enabled for this account」と表示されます。プランで有効にするにはサポートに連絡してください。

#### プラットフォームを選択（任意）

**Generate a Dynamic Registration URL** の下で、**Platform** ドロップダウンを使い、どの LMS に接続するかを FastComments に伝えます:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Other LTI 1.3 platform

**Auto-detect** のままにしておくこともできます。登録中に LMS の openid-configuration からプラットフォームが読み取られます。ドロップダウンは生成される構成の表示ラベルを設定するためだけに使われます。

#### URL を生成する

**Generate URL** をクリックします。FastComments はワンタイム登録トークンを作成し、次のような URL を表示します:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

コピーしてください。この URL は:

- は **単回使用** です — 一度 LMS が正常に呼び出すとトークンは消費されます。
- 使用されない場合、**30 分**後に期限切れになります。
- 非公開にしてください — URL を持っている人はその 30 分以内にあなたのテナントに対してツールを登録できます。

#### 既存の構成

登録が正常に完了すると、新しい構成が同じページの **既存の構成** テーブルに表示され、プラットフォーム、Issuer、Client ID、およびステータスが確認できます。登録解除が必要になった場合は、このテーブルから構成を削除できます。