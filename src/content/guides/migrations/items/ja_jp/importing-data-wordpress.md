私たちの[WordPressプラグイン](https://wordpress.org/plugins/fastcomments/)には、強力なUIベースのインポート機能があります。プラグインをインストールすると、WordPressのインストール環境とFastCommentsを連携させ、既存のコメントデータをコピーする手順をガイドします。

**手動で何かをコピーしたりダウンロードしたりする必要はありません。**

移行プロセスの進行状況は、移行中にUIを通じて表示されます。ほとんどの移行は数分で完了します。

この仕組みは、移行中にWordPressのインストール環境に過度な負荷をかけないように設計されています。

サイトをWordPressから移行する場合は、プラグインを使用する代わりに、WordPressのXMLまたはCSVエクスポートをインポートすることができます。詳細は[Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress)をご覧ください。

### CloudFlareとファイアウォール

WordPressの自動セットアップを機能させるには、WordPressのインストール環境への呼び出しを行う必要があります。Cloudflareなどのファイアウォールが私たちをブロックし、統合が失敗する場合があります。そのような場合は、統合のために[許可リストに登録していただくIPアドレスのセットを提供できます](https://fastcomments.com/auth/my-account/help)。

### データの所有権

WordPressの移行の場合、新規または更新されたコメントデータは、バックグラウンドで自動的にWordPressのインストール環境に同期されます。つまり、コメントはFastComments自体によって提供され、WordPressのデプロイメントの負荷を軽減しますが、私たちは**また**バックアップとしてそれらをデータベースに保存しています。これはまた、FastCommentsから他のサービスに切り替えたい場合でも、データはすでに移行されており、最新の状態であることを意味します。