1. FastComments にサインインし、<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">あなたの LTI 1.3 構成ページ</a>に移動します。
2. (オプション) 接続元のプラットフォームを **Platform** ドロップダウンから選択します - 表示ラベルが設定されますが、Auto-detect でも問題ありません。
3. **Generate URL** をクリックします。ワンタイムの **Registration URL** が表示されます（有効期限は30分、単回使用）。
4. LMS で LTI 1.3 の Dynamic Registration 画面を開き、URL を **Tool initiation registration endpoint**（または同等の）フィールドに貼り付けて送信します。
5. LMS が FastComments にコールバックし、鍵を交換して統合を作成します。完了するとポップアップは自動で閉じます。
6. FastComments に戻ると、新しい構成が **Existing Configurations** テーブルに表示されます。ツールは LMS のコース内で利用可能になります。