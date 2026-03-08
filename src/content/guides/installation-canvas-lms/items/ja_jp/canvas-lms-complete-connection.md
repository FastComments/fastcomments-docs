#### FastComments に Client ID を入力

戻って <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">FastComments の LTI 設定</a> を開きます。ウィザードは **Step 2: Connect** の状態になっているはずです。

Canvas からコピーした **Client ID** を **Client ID** フィールドに貼り付けます。LMS が提供する場合は任意で **Deployment ID** を入力します。

**Save & Continue** をクリックします。

#### 統合を有効にする

ウィザードは **Step 3: Go Live** に進みます。設定の概要が表示されます（名前、プラットフォームの URL、Client ID、そして deployment ID）。

詳細を確認したら、**Enable Integration** をクリックして LTI 接続を有効にします。

有効化後、ウィザードは **Management View** を表示し、ここで設定の編集、すべての LTI URL の表示、または追加のデプロイメントの追加ができます。

#### Canvas に外部アプリをインストールする

Canvas で **Admin** > アカウントを選択 > **Settings** > **Apps** タブ に移動します。

**+ App** をクリックして次のように設定します:

1. **Configuration Type** を **By Client ID** に設定します。
2. Developer Keys テーブルから **Client ID** を貼り付けます。
3. **Submit** をクリックします。
4. プロンプトが表示されたらインストールを確認します。

これで FastComments はアカウントレベルにインストールされ、すべてのコースで利用可能になります。