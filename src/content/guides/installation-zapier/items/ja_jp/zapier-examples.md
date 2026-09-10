## Zap の例

数分で設定できるワークフローの例です。

**新しいコメントの通知を受け取る。** 「New Comment」トリガーの後、Slack の「Send Channel Message」または Discord の「Send Channel Message」を使用します。コメント投稿者の名前、コメント本文、ページ URL をメッセージにマッピングします。ドメインフィルターを追加して、サイトごとに異なるチャンネルへ通知できます。

**すべてのコメントを記録する。** 「New Comment」トリガーの後、Google Sheets の「Create Spreadsheet Row」を使用します。削除されたコメント用に別の Zap を追加し、コメント ID を含む行を追記することで、シートを監査ログとしても活用できます。

**コメントが承認されたときに作者にメールを送る。** 「Updated Comment」トリガーに Zapier のフィルターで Approved が true のものだけを通し、次に Gmail の「Send Email」を使用します。Updated Comment はすべての変更で発火するため、フィルターが承認時のみ Zap が動作するようにします。

**コメント投稿者を CRM やメーリングリストに追加する。** 「New Comment」トリガーの後、HubSpot の「Create or Update Contact」または Mailchimp の「Add or Update Subscriber」を使用し、コメント投稿者のメールアドレスで連絡先を作成または更新します。マーケティングリストに追加する前に、プライバシーポリシーと現地法を遵守してください。

**フォームからコメントを作成する。** Typeform または Google Forms の「New Response」トリガーの後、FastComments の「Create Comment」を使用し、サイトでテストimonial に使用しているページ URL ID を指定します。Approved をチェックしないようにして、公開前に各コメントをレビューできます。

**お知らせをフィードに投稿する。** Zapier の RSS 「New Item in Feed」トリガーの後、Create Feed Post を使用し、アイテムのタイトル、コンテンツ、リンクを投稿します。

**メンバーを SSO ユーザーとしてプロビジョニングする。** Memberstack、Memberful、または独自の webhook を使用し、次に「Find SSO User」を実行し、続いて「Create SSO User」を「find or create」モードで実行します。

**通報されたコメントをエスカレーションする。** 「Updated Comment」トリガーに、フラグ数が 0 より大きいものをフィルタリングし、次に Trello の「Create Card」または Linear の「Create Issue」を使用して、コメント ID とモデレーションページへのリンクを添付します。

**ページが公開されたらすぐに公開する。** WordPress または Ghost の「New Post」トリガーの後、Create Page を使用し、投稿の URL を指定します。これにより、最初のコメントが来る前にページが一覧に表示され、アクセス制限が適用されます。

**削除されたコメントをアーカイブする。** 「Deleted Comment」トリガーの後、Airtable の「Create Record」を使用し、完全なコメント内容を保存してコンプライアンス保持に備えます。