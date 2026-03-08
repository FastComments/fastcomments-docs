#### FastComments にクライアントIDを入力する

FastComments の <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">マイアカウント > Canvas LTI 設定</a> に戻ります。ウィザードは **ステップ2：接続** になっているはずです。

Canvas からコピーした **Client ID** を **Client ID** フィールドに貼り付けます。LMS が提供する場合は、任意で **Deployment ID** を入力してください。

**Save & Continue** をクリックします。

#### 統合を有効にする

ウィザードは **ステップ3：公開** に進みます。構成の概要（名前、プラットフォームの URL、Client ID、Deployment ID）が表示されます。

詳細を確認してから、LTI 接続を有効にするために **Enable Integration** をクリックします。

有効化すると、ウィザードは **Management View** を表示します。ここで構成を編集したり、すべての LTI URL を表示したり、追加のデプロイメントを追加したりできます。

#### Canvas に外部アプリをインストールする

Canvas で、**Admin** > 対象のアカウントを選択 > **Settings** > **Apps** タブに移動します。

**+ App** をクリックし、次のように設定します：

1. **Configuration Type** を **By Client ID** に設定します。
2. Developer Keys テーブルから **Client ID** を貼り付けます。
3. **Submit** をクリックします。
4. プロンプトが表示されたらインストールを確認します。

これで FastComments はアカウントレベルにインストールされ、すべてのコースで利用可能になります。