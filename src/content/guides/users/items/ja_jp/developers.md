あなたが`Administrators`にしたくない開発者には、`Administrator`ユーザーを作成することを検討してください
次の権限:

1. Analytics Admin
2. Customizations Admin
3. Data Management Admin
4. Comment Moderation Admin
5. API/SSO Admin

この権限セットにより、開発者はFastCommentsを設定するために必要なすべてを得られるとともに
システムが正しく動作しているかを確認するために必要な可視性も得られます。

これらの権限の理由は次のとおりです:

1. **Analytics Admin**: これは FastComments の使用状況をデバッグするために使用できます。
2. **Customizations Admin**: これはコメントウィジェットにカスタムスタイルを適用するために必要です。
3. **Data Management Admin**: これはインポートおよびエクスポートの管理やウェブフックの設定に必要です。
4. **Comment Moderation Admin**: これは少なくともセットアップ時にコメントデータを閲覧するために必要です。
5. **API/SSO Admin**: これにより、彼らは当社のプラットフォームからAPI keysを直接取得できます。私たちは
これが`Administrator`がそれをコピーしてAPI Secretをメールで送るよりも安全であると考えていますが
   メールはあまり安全ではない可能性があります。