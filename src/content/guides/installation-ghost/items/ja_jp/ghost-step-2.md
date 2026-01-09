---
zipファイルをダウンロードしたので、フォルダに解凍します。デフォルトの `casper.zip` をダウンロードし、Windows の `Downloads\casper` に解凍しました。

次に、LTS またはそれ以降のバージョンの NodeJS がインストールされていることを確認してください。入手はこちら: https://nodejs.org/en/download/

NodeJS がインストールされたら、コードエディタをインストールします。

当社では Webstorm を推奨（および使用）しています。30日間のトライアル（クレジットカード不要）で入手できます: https://www.jetbrains.com/webstorm/

次に良い無料の選択肢はおそらく Visual Studio Code でしょう: https://code.visualstudio.com/download

エディタのセットアップが完了し、テーマフォルダをエディタで開いたら、IDE のターミナルを開き、次を実行します:

[inline-code-attrs-start title = 'テーマをインストール'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

出力は次のようになります（警告は無視して構いません）:

<div class="screenshot white-bg">
    <div class="title">npm install の成功時の出力</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="npm install の成功時の出力" />
</div>

これにより、後で実行するコマンドのためにテーマの依存関係が設定されます。また、エクスポートはテーマの依存関係がインストールされていることに依存するため、インポートし直す際に正しく動作するには依存関係が必要です。

---