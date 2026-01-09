Custom Code タブの **Footer** セクションに、以下のコードを貼り付けてください:

[inline-code-attrs-start title = 'Typeflo.io ライブコメントのコードスニペット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js" async></script>
<script>
    (function () {
        function load() {
            let target = null;
            let lastInstance;
            if (document.querySelector('.fastcomments-widget')) {
                setTimeout(load, 1000);
                return;
            }
            if (lastInstance) {
                lastInstance.destroy();
            }
            if (window.FastCommentsUI) {
                const newElement = document.createElement('div');
                newElement.classList.add('fastcomments-widget');
                const subscribeSection = document.querySelector('.nc-SectionSubscribe2');
                if (subscribeSection) {
                    subscribeSection.parentNode.insertBefore(newElement, subscribeSection);
                    target = newElement;
                } else {
                    const fullWidthSection = document.querySelector('.container.w-full');
                    if (fullWidthSection) {
                        fullWidthSection.prepend(newElement);
                        target = newElement;
                    }
                }
            }
            if (target) {
                lastInstance = FastCommentsUI(target, {
                    "tenantId": "demo"
                });
            }
            setTimeout(load, 1000);
        }

        load();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">フッターセクションにコードを貼り付ける</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="FastComments コードをフッターセクションに貼り付ける" />
</div>

コードを貼り付けたら、変更を適用するために **Save** ボタンをクリックしてください。

注意: このコードには、FastCommentsウィジェットを Typeflo.io のブログ記事上の最適な位置に動的に配置するロジックが含まれています。他のコードスニペットは Typeflo.io のレイアウトでは正しく動作しません。

サインアップ後に、`'demo'` を実際の FastComments テナントID に置き換えることを忘れないでください。FastComments.com にログインしている場合は、すでに置き換えられているはずです。