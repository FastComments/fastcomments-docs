### 概要

FastComments Collab Chat は標準の FastComments コメントウィジェットを拡張しており、ベースウィジェットのすべての設定オプションを継承しつつ、テキスト注釈に特有のいくつかのオプションを追加します。

### 必須の設定

#### tenantId

FastComments の Tenant ID が必要です。これは [FastComments ダッシュボード](https://fastcomments.com/auth/my-account/api-secret) で確認できます。

[inline-code-attrs-start title = "構成例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Collab Chat 固有のオプション

#### urlId

デフォルトでは、Collab Chat はページの URL、要素への DOM パス、および選択されたテキスト範囲に基づいて各会話の一意の識別子を生成します。カスタムの `urlId` でこれを上書きできます。

[inline-code-attrs-start title = "構成例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

これは、URL 構造が変わる可能性があるが会話を維持したい場合、または注釈を複数ページで共有したい場合に便利です。

#### topBarTarget

ユーザー数とディスカッション数を表示するトップバーの表示を制御します。トップバーを完全に無効にするには `null` を設定するか、特定の場所にレンダリングするための DOM 要素を指定します。

[inline-code-attrs-start title = "構成例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// トップバーを無効化
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// カスタム位置にトップバーをレンダリング
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

ページにダークな背景がある場合にダークモードのスタイリングを有効にします。この検出は自動ですが、上書きしたい場合もあります。

[inline-code-attrs-start title = "構成例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

コメント数が変化するたびに呼び出されるコールバック関数です。バッジやページタイトルなどの UI 要素を更新するのに便利です。

[inline-code-attrs-start title = "構成例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### 継承された設定オプション

Collab Chat は標準のコメントウィジェットを拡張しているため、ベースの FastComments ウィジェットの任意の設定オプションを使用できます。ここではよく使われるオプションを示します:

#### locale

ウィジェット UI の言語を設定します。FastComments は多数の言語をサポートしています。

[inline-code-attrs-start title = "構成例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // スペイン語
});
[inline-code-end]

#### readonly

すべての会話を読み取り専用にします。ユーザーは既存の注釈を閲覧できますが、新しい注釈を作成したり返信したりすることはできません。

[inline-code-attrs-start title = "構成例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

シングルサインオンを使用して認証システムと統合します。

[inline-code-attrs-start title = "構成例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO の設定
    }
});
[inline-code-end]

認証オプションの詳細については SSO ドキュメントを参照してください。

#### maxReplyDepth

返信の階層の深さ（ネストのレベル）を制御します。デフォルトでは Collab Chat はこれを 0 に設定しており、すべてのコメントはフラット（ネストされた返信なし）になります。スレッド化された会話にしたい場合はこれを変更できます。

[inline-code-attrs-start title = "構成例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // 3レベルのネストを許可
});
[inline-code-end]

### 内部設定

これらのオプションは Collab Chat によって自動的に設定され、上書きすべきではありません：

The `productId` is automatically set to `3` for Collab Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly.

### 完全な例

複数の設定オプションを組み合わせた例を以下に示します：

[inline-code-attrs-start title = "構成例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(document.getElementById('article'), {
    tenantId: 'demo',
    urlId: 'my-article-v2',
    hasDarkBackground: false,
    locale: 'en',
    topBarTarget: document.getElementById('custom-header'),
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) My Article` : 'My Article';
    },
    sso: {
        // あなたの SSO 設定
    },
    maxReplyDepth: 1
});
[inline-code-end]

For a complete list of all available configuration options inherited from the base widget, see the main FastComments configuration documentation.