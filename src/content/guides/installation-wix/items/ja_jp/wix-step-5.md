次に、コメントスレッドが現在のページに応じて変わるように設定します。これにより、ユーザーは現在表示されているコンテンツについて議論できるようになります。

以下の手順を行わないと、サイト全体で1つのグローバルなコメントスレッドしか持てず、あまり役に立ちません。

#### Dev Mode

この機能を追加するには、Wixが `Dev Mode` と呼ぶモードに入る必要があります。

画面上部の `Dev Mode` オプションをクリックしてください。

<div class="screenshot white-bg">
    <div class="title">Dev Mode を有効にする</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Dev Mode を有効にする" />
</div>

#### 要素のIDを設定

これを実現するためにカスタムコードを追加しますが、まず参照するための新しい埋め込み要素にIDを付ける必要があります。

名前は `fastcomments` にしましょう。

追加した新しい埋め込み要素をクリックすると、Dev Mode の右下に `html1` のような値を持つ ID フィールドが表示されます：

<div class="screenshot white-bg">
    <div class="title">ID フィールド</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="ID フィールド" />
</div>

これを `fastcomments` に変更して Enter を押します：

<div class="screenshot white-bg">
    <div class="title">ID を設定</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="ID を設定" />
</div>

これで、コメント領域に現在表示しているページを伝えるカスタムコードを追加できます。

画面下部に次のようなコードエディタが表示されます：

<div class="screenshot white-bg">
    <div class="title">エディタを開く</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="エディタを開く" />
</div>

以下のコードをコピーしてそこに貼り付けてください：

[inline-code-attrs-start title = 'Wix コメントナビゲーション スニペット'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import wixLocation from 'wix-location';

$w.onReady(function () {
    function updateFastCommentsLocation() {
        try {
            const url = (wixLocation.baseUrl + '/' + wixLocation.path).replace(/,/g, '/');
            $w('#fastcomments').postMessage({'action': 'reload', 'url': url});
        } catch (err) {
            console.error('Wix -> FastComments Error', err);
        }
    }

    updateFastCommentsLocation();
    wixLocation.onChange( () => {
        updateFastCommentsLocation();
    });
});
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">ナビゲーションコードを追加</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="ナビゲーションコードを追加" />
</div>