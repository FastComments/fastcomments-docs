次に、現在のページに応じてコメントスレッドが切り替わるように設定し、表示中のコンテンツについてユーザーが議論できるようにしましょう。

以下の手順を行わない場合、サイト全体で1つのグローバルコメントスレッドしか持てず、あまり有用ではありません。

#### Dev Mode

この機能を追加するには、Wix が `Dev Mode` と呼ぶものに入る必要があります。

画面上部の `Dev Mode` オプションをクリックしてください。

<div class="screenshot white-bg">
    <div class="title">Dev Mode を有効にする</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Dev Mode を有効にする" />
</div>

#### 要素 ID の設定

これを実現するためにカスタムコードを追加しますが、まず新しい埋め込み要素に参照用の ID を付ける必要があります。

`fastcomments` と呼ぶことにしましょう。

追加した新しい埋め込み要素をクリックすると、Dev Mode で画面右下に `html1` のような値が入った ID フィールドが表示されるはずです:

<div class="screenshot white-bg">
    <div class="title">ID フィールド</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="ID フィールド" />
</div>

これを `fastcomments` に変更して Enter キーを押してください:

<div class="screenshot white-bg">
    <div class="title">ID の設定</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="ID の設定" />
</div>

これで、表示中のページをコメントエリアに伝えるカスタムコードを追加できます。

画面下部に、次のようなコードエディタが表示されているはずです:

<div class="screenshot white-bg">
    <div class="title">エディタを開く</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="ID の設定" />
</div>

以下のコードをコピーして貼り付けてください:

[inline-code-attrs-start title = 'Wix コメントナビゲーションスニペット'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">ナビゲーションコードの追加</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="ナビゲーションコードの追加" />
</div>