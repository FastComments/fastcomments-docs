### 概要

FastComments Collab Chat は標準の FastComments コメントウィジェットを拡張したものであり、ベースウィジェットのすべての設定オプションを継承しつつ、テキストアノテーションに特化したいくつかのオプションを追加しています。

### 必須の設定

#### tenantId

FastComments のテナント ID(tenantId)が必要です。[FastComments ダッシュボード](https://fastcomments.com/auth/my-account/api-secret) から確認できます。

[inline-code-attrs-start title = "設定例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Collab Chat 固有のオプション

#### urlId

デフォルトでは、Collab Chat はページ URL、要素への DOM パス、選択されたテキスト範囲に基づいて、会話ごとに一意の識別子を生成します。カスタムの `urlId` を指定してこれを上書きできます。

[inline-code-attrs-start title = "設定例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

これは、URL 構造が変更される可能性があるが同じ会話を維持したい場合や、複数のページ間でアノテーションを共有したい場合に便利です。

#### topBarTarget

ユーザー数とディスカッション数を表示するトップバーの表示を制御します。`null` を設定するとトップバーを完全に無効化でき、DOM 要素を指定すると特定の場所にレンダリングできます。

[inline-code-attrs-start title = "設定例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// トップバーを無効化
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// トップバーをカスタム位置にレンダリング
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

ページの背景が暗い場合にダークモードのスタイリングを有効にします。この検出は自動的に行われますが、手動で上書きしたい場合もあるでしょう。

[inline-code-attrs-start title = "設定例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

コメント数が変化するたびに呼び出されるコールバック関数です。バッジやページタイトルなどの UI 要素の更新に便利です。

[inline-code-attrs-start title = "設定例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### 継承される設定オプション

Collab Chat は標準のコメントウィジェットを拡張しているため、ベースの FastComments ウィジェットの任意の設定オプションを使用できます。よく使われるオプションの例を以下に示します:

#### locale

ウィジェット UI の言語を設定します。FastComments は数十の言語をサポートしています。

[inline-code-attrs-start title = "設定例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // スペイン語
});
[inline-code-end]

#### readonly

すべての会話を読み取り専用にします。ユーザーは既存のアノテーションを閲覧できますが、新規作成や返信はできません。

[inline-code-attrs-start title = "設定例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso と simpleSSO

シングルサインオン(SSO)を使用して、お使いの認証システムと統合できます。

[inline-code-attrs-start title = "設定例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO 設定
    }
});
[inline-code-end]

認証オプションの詳細については、SSO ドキュメントを参照してください。

#### maxReplyDepth

返信が何階層までネストできるかを制御します。デフォルトでは Collab Chat はこれを 0 に設定しており、すべてのコメントがフラット(ネストされた返信なし)になります。スレッド形式の会話を希望する場合は変更できます。

[inline-code-attrs-start title = "設定例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // 3 階層までのネストを許可
});
[inline-code-end]

### 内部設定

これらのオプションは Collab Chat によって自動的に設定されるため、上書きしないでください:

Collab Chat では `productId` が自動的に `3` に設定されます。チャットウィンドウ機能を提供するために `floating-chat` 拡張が自動的に読み込まれます。また、ウィジェットはモバイルデバイス(幅 768px 未満の画面)を自動的に検出し、それに応じて UI を調整します。

### 完全な例

複数の設定オプションを組み合わせた例を以下に示します:

[inline-code-attrs-start title = "設定例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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
        // お使いの SSO 設定
    },
    maxReplyDepth: 1
});
[inline-code-end]

ベースウィジェットから継承されるすべての設定オプションの一覧については、FastComments のメイン設定ドキュメントを参照してください。