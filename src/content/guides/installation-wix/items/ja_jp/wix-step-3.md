この例は Wix と互換性を持たせるためにカスタムコードを使用しています。**他のチュートリアルの FastComments コードスニペットは使用できません。**

カスタム HTML ダイアログを追加するには、`Enter Code` をクリックして `HTML` を選択し、フォームを開いてください：

<div class="screenshot white-bg">
    <div class="title">ステップ 3: HTML ダイアログを開く</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="ステップ 3: HTML ダイアログを開く" />
</div>

以下の HTML スニペットをコピーしてダイアログに貼り付け、Update をクリックしてください：

[inline-code-attrs-start title = 'Wix コメントのコードスニペット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const config = {
            tenantId: "demo"
        };
        const instance = FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        window.onmessage = (event) => {
            if (event.data) {
                if (event.data.action === 'reload') {
                    console.log('Updating FastComments:', event.data.url);
                    config.urlId = event.data.url;
                    config.url = event.data.url;
                    instance.update(config);
                }
            }
        }
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">ステップ 3: 貼り付けて保存</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="ステップ 3: 貼り付けて保存" />
</div>

これでコメントウィジェットの非常に小さなバージョンが表示されるはずです：

<div class="screenshot white-bg">
    <div class="title">ステップ 3: 結果</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="ステップ 3: 結果" />
</div>

次に、これの位置とサイズを調整してページに合わせます。

---