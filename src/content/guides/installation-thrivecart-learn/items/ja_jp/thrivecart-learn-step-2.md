ステップ2では、コードスニペットをコピーします。50行目に「demo」と表示されていないことを確認してください。ここにはテナントIDが入っている必要があります。通常は自動的に入力されています。

次に、ThriveCart Learn 専用の FastComments コードスニペットをコピーしましょう。

ThriveCart との連携には多くの機能があるため、スニペットはかなり大きくなっています。コードスニペットの右上にある「コピー」ボタンをクリックするだけです:

[inline-code-attrs-start title = 'ThriveCart Learn+ Comments Code'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let attemptsRemaining = 10;

        function tryLoad() {
            const simpleSSO = {optedInNotifications: true, optedInSubscriptionNotifications: true};
            let isAuthenticated = false;
            let profileLink = document.querySelector('.thrivecart-courses-header-profile-link');
            if (!profileLink) {
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // プレビューではクラスが異なります。
            }
            // ThriveCart が id を変更した場合に備えた、幅広いメール入力フィールドのセレクター。
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // メールが利用できないため、プレビューが機能するようにします。
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // インターネットが遅い場合に備えて、5回試行後は待機時間を長くします。
            }
            if (profileLink) {
                // ThriveCart が画像クラスのセレクターを変更した場合に備えて、生の "img" クエリを使用します。
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // ThriveCart がプロフィール名の表示方法を変更した場合に備えて、innerText を使用します。
                if (profileLink.innerText) {
                    isAuthenticated = true;
                    simpleSSO.username = profileLink.innerText;
                } else {
                    const bold = profileLink.querySelector('b');
                    if (bold && bold.innerText) {
                        isAuthenticated = true;
                        simpleSSO.username = bold.innerText;
                    }
                }
            } else {
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (user name/avatar). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user profile info found - waiting and trying again.');
                attemptsRemaining--;
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // インターネットが遅い場合に備えて、5回試行後は待機時間を長くします。
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // TC は同じページへの複数のリンクを使用することがあるため、重複を排除します。
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // マーケティングパラメータとドメイン名を取り除きます
                url = window.location.pathname;
            }

            if (url) {
                url = url.replace('/starte-hier', '');
                url = url.replace('/start-here', '');
            }

            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: 'demo',
                urlId: url,
                simpleSSO: isAuthenticated ? simpleSSO : null
            });
        }

        tryLoad();

        function getPathnameFromUrl(url) {
            try {
                const parsedUrl = new URL(url);
                // マーケティングパラメータとドメイン名を取り除きます
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // 現在の値をデフォルトにして、少なくとも動作するようにします
            }
        }

    })();
</script>
[inline-code-end]

次に、ThriveCart エディターの左側にあるコードブロックに貼り付けてください。以下のようになります:

<div class="screenshot white-bg">
    <div class="title">コード追加済み</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Code Added" />
</div>

これで完了です!あとは公開するだけです:

<div class="screenshot white-bg">
    <div class="title">公開</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Publish" />
</div>

これで完了です!プレビュー時にコースにコメントボックスが表示され、実際のユーザーは**サインインやユーザー名/メールの再入力をすることなく**コメントを残せるようになります。

### テストに関する注意!

匿名コメントが無効になっている場合(デフォルトで無効です)、`Preview` モードで `John Smith` ユーザーとしてコメントを残すことはできません。デフォルトの `John Smith` ユーザーにはメールアドレスがないため、認証エラーが発生します。テストしたい場合は、クーポンコードを使用して、実際のユーザーのようにサイトを通過することをお勧めします。