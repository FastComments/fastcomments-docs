---
1. <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">あなたの FastComments LTI Config</a> にアクセスします。
2. **Configuration Name** と **Platform URL** を入力します（例: `https://yourschool.instructure.com`）。有効にする **Placements** を選択します（Assignment View および/または Editor Button — どちらもデフォルトでオンになっています）。**Create Configuration** をクリックします。ウィザードは Step 2 に進み、**Configuration URL** を表示します。
3. Canvas で、**Admin > Developer Keys > + Developer Key > LTI Key** に移動します。**Method** を "Enter URL" に設定し、Configuration URL を貼り付けます。キーを保存し、その後 **State** を **ON** に設定し、プロンプトが表示されたら **Allow** をクリックします。
4. Canvas の Developer Keys テーブルから **Client ID** の番号をコピーします。FastComments に戻り、それを **Client ID** フィールドに貼り付け、**Save & Continue** をクリックします。
5. 設定の要約を確認し、公開するには **Enable Integration** をクリックします。
6. Canvas に External App をインストールします（**Admin > Settings > Apps > + App > By Client ID**）。コメントは課題の下に自動的に表示され、講師はリッチコンテンツエディタのツールバーボタンから Pages、Quizzes、Announcements に FastComments を埋め込むことができます。

---