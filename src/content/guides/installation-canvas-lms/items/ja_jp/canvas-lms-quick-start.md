1. FastComments にログインし、<a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">マイアカウント > Canvas LTI 設定</a> に移動します。
2. **設定名** と **プラットフォーム URL**（例: `https://yourschool.instructure.com`）を入力します。有効にする **配置 (Placements)**（Assignment View および/または Editor Button — どちらもデフォルトでオン）を選択します。**設定を作成** をクリックします。ウィザードがステップ2に進み、**設定 URL** が表示されます。
3. Canvas で **Admin > Developer Keys > + Developer Key > LTI Key** に移動します。**Method** を「URL を入力」に設定し、設定 URL を貼り付けます。キーを保存し、次に **State** を **ON** にして、表示されたら **Allow** をクリックします。
4. Canvas の Developer Keys テーブルから **Client ID** の番号をコピーします。FastComments に戻り、**Client ID** フィールドに貼り付けて **保存して続行** をクリックします。
5. 設定の概要を確認し、**統合を有効にする** をクリックして公開します。
6. Canvas に外部アプリをインストールします（**Admin > Settings > Apps > + App > By Client ID**）。コメントは課題の下に自動的に表示され、講師はリッチコンテンツエディタのツールバーのボタンを使って Pages、Quizzes、Announcements に FastComments を埋め込むことができます。