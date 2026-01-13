小さなコードスニペットで、FastCommentsとMemberSpaceを簡単に接続できます:

[inline-code-attrs-start title = 'FastComments MemberSpace Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo';
        const ALLOW_ANON = false;
        const LOGIN_URL = 'https://example.com/login';
        const PLAN_DISPLAY_LABELS = {
            'VIP Plan': 'VIP'
        };
        let lastInstance;

        function tick() {
            if (!window.MemberSpace) {
                return setTimeout(tick, 100);
            }
            if (!window.FastCommentsUI) {
                return setTimeout(tick, 100);
            }
            const target = document.getElementById('fastcomments-widget');
            if (!target) {
                return setTimeout(tick, 100);
            }
            const data = MemberSpace.getMemberInfo();
            if (data.isLoggedIn && data.memberInfo) {
                lastInstance = FastCommentsUI(target, {
                    tenantId: tenantId,
                    urlId: window.location.pathname,
                    simpleSSO: {
                        displayLabel: getDisplayLabel(data.memberInfo),
                        username: data.memberInfo.name,
                        email: data.memberInfo.email,
                        avatar: data.memberInfo.profileImageUrl
                    }
                });
            } else if (lastInstance) {
                lastInstance.destroy();
                lastInstance = FastCommentsUI(target, {
                    tenantId: tenantId,
                    urlId: window.location.pathname,
                    simpleSSO: getAnonSSOConfig()
                });
            }
        }

        function getAnonSSOConfig() {
            if (ALLOW_ANON) {
                return undefined;
            }
            return {
                loginURL: LOGIN_URL
            };
        }

        function getDisplayLabel(memberInfo) {
            if (!memberInfo.memberships) {
                return;
            }
            for (const membership of memberInfo.memberships) {
                const label = PLAN_DISPLAY_LABELS[membership.name];
                if (label) {
                    return label
                }
            }
        }

        tick();
    })();
</script>
[inline-code-end]

ユーザーがMemberStack経由でログインした状態であなたのサイトやアプリケーションにアクセスすると、自動的にFastCommentsにログインし、そのコメントは
`Verified`としてマークされます。

また、上記の例では、`VIP Plan`という名前のサブスクリプションプランがある場合、ユーザー名の横に`VIP`バッジを表示します。例を編集して
プランを追加できます。ご質問がありましたらサポートにお問い合わせください。

### 匿名コメントを許可する

**匿名コメント**も許可したい場合は、ALLOW_ANONを次のようにtrueに設定します:

    const ALLOW_ANON = true;

また、`https://example.com/login`を、ユーザーが`Login`ボタンをクリックしたときに移動させたい場所に変更することを忘れないでください:

この方法で、メンバーサイトにログインしていないユーザーも、名前とメールアドレスを入力してコメントするオプションが得られます。
