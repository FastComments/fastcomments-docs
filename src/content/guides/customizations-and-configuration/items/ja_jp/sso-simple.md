[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Simple SSO を使用すると、コメントウィジェットにユーザー情報を提供できるため、ユーザーはコメントする際にユーザー名やメールアドレスを入力する必要がなくなります。

Simple SSO は以下のように設定できます：

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

ユーザーはログイン状態となり、裏で SSO ユーザーが作成されます。API から取得された場合、ユーザーの `createdFromSimpleSSO` は `true` に設定されます。

注意点：

- メールアドレスは Simple SSO のユニークな識別子です。
- Simple SSO でメールアドレスを提供することは必須ではありませんが、デフォルトではコメントは「未確認」と表示されます。<b>メールアドレスが提供されない場合、ユーザーは完全に認証されません。</b>
- **NEW** 2022年1月以降: ユーザー名は fastcomments.com 全体でユニークである必要はなくなりました。
- メールアドレスが提供され、かつユーザーが Secure SSO から元々作成されていない場合、Simple SSO は SSO ユーザーを自動的に作成・更新できます。Simple SSO のペイロードで Secure SSO または API によって作成されたユーザーが指定された場合、そのユーザーは変更されず、訪問者には別のユーザーが作成されます。
- `badgeConfig` プロパティでユーザーにバッジを指定できます。`badgeIds` 配列にはユーザーに関連付けるグローバルバッジの ID が含まれます。`pageBadgeIds` 配列には現在のページ（`urlId`）にスコープされたバッジ ID が含まれ、これらのバッジは割り当てられたページでのみ表示されます。`override` が `true` に設定されている場合、既存の表示バッジ（グローバルとページスコープ）は置き換えられます（それぞれ独立して上書き）。`false` の場合は既存のバッジに追加されます。

#### Simple SSO 設定

Simple SSO は常に利用可能です。2 つの設定で FastComments がどの程度信頼するかを制御します。これらは <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">カスタマイズルール</a> 内で SSO を検索することで見つけられます。デフォルトでは両方ともオフになっており、"Require SSO To View Comments" がオンかオフかに関わらず適用されます。

これらの設定が追加された時点で既に Simple SSO を使用していたアカウントは、両方ともオンになっているため、変更はありません。

ページに最も適合するカスタマイズルールのみが適用されるため、複数のルールがある場合はそれぞれに設定してください。他のルールでオンになっている場合、新しいルールはそれらがオンの状態で開始されます。

##### Simple SSO によるコメントの自動検証

この設定がオンの場合、Simple SSO ユーザーからのコメントはすぐに検証されます。

この設定がオフの場合、Simple SSO ユーザーからのコメントは「未確認」と表示され、ユーザーが Simple SSO ペイロード内のメールアドレスに送信されたリンクをクリックするまで検証されません。検証済みコメントのみ自動承認する設定の場合も同様にリンクを待ちます。ペイロードにメールアドレスが含まれていない場合、コメントは未確認のままです。

この設定はコメントにのみ影響し、Simple SSO ユーザーからの投票は常に即座にカウントされます。

##### Simple SSO 管理者の許可

この設定がオンの場合、Simple SSO ユーザーはアカウントの管理者またはモデレーターになることができます：

- 管理者またはモデレーター権限を持つ Simple SSO ユーザーはその権限を保持します。
- 管理者またはモデレーターのメールアドレスと同じメールアドレスを持つ Simple SSO ユーザーは、その権限を取得します。

この設定がオフの場合、Simple SSO はこれらの権限を付与しません。権限を持つユーザーが指定された Simple SSO ペイロードは、権限のない別のユーザーを作成し、元のユーザーは変更されません。

Simple SSO ペイロードは署名されていないため、この設定がオンの場合、管理者のメールアドレスやユーザー ID を知っている誰でもその管理者として振る舞うことができます。サイトでこれが問題になる場合は、オフにして管理者には Secure SSO を使用してください。

Secure SSO または API によって作成されたユーザーは、Simple SSO がオンかオフかに関わらず、Simple SSO からはアクセスできません。

---