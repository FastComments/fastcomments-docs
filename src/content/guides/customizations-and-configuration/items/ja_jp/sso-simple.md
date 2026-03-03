[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Simple SSO を使うと、コメントウィジェットにユーザー情報を提供できるため、ユーザーはコメント時にユーザー名やメールアドレスを入力する必要がなくなります。

Simple SSO は次のように設定できます：

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

ユーザーはログインした状態になり、裏で SSO ユーザーが作成されます。API から取得された場合、ユーザーの `createdFromSimpleSSO` は `true` に設定されます。

Notes: 

- メールアドレスは Simple SSO における一意の識別子です。
- Simple SSO でメールアドレスを提供する必要はありませんが、デフォルトではコメントは "Unverified" と表示されます。 <b>メールアドレスが提供されない場合、ユーザーは完全に認証されません。</b>
- **NEW** 2022年1月以降: fastcomments.com 全体でユーザー名が一意である必要はありません
- メールアドレスが提供され、かつユーザーが元々 Secure SSO から作成されていなかった場合、Simple SSO は SSO ユーザーを自動的に作成および更新できます。
- ユーザーに付与するバッジは `badgeConfig` プロパティで指定できます。`badgeIds` 配列にはユーザーに関連付けるグローバルバッジの ID が含まれます。`pageBadgeIds` 配列には現在のページ（`urlId`）にスコープされたバッジ ID が含まれます — これらのバッジは割り当てられたページでのみ表示されます。`override` が `true` に設定されている場合、既存の表示中のバッジを置き換えます（グローバルとページスコープは独立して上書きされます）；`false` の場合は既存のバッジに追加されます。

---