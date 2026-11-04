### 概要

FastComments Image Chat は標準の FastComments コメントウィジェットを拡張したものであり、ベースウィジェットのすべての設定オプションを継承しつつ、画像アノテーションに特化したいくつかのオプションを追加しています。

### 必須の設定

#### tenantId

FastComments のテナント ID(tenantId)が必要です。[FastComments ダッシュボード](https://fastcomments.com/auth/my-account/api-secret) から確認できます。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat 固有のオプション

#### urlId

デフォルトでは、Image Chat はページ URL、画像のソース、X/Y 座標に基づいて会話ごとに一意の識別子を生成します。カスタムの `urlId` を指定してこれを上書きできます。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

これは、URL 構造が変わる可能性があるが同じ会話を維持したい場合や、複数のページ間でアノテーションを共有したい場合に便利です。

#### chatSquarePercentage

クリック可能なチャットマーカーのサイズを画像幅に対する割合で制御します。デフォルトは 5% で、各マーカーは画像幅の 5% になります。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // より大きく目立つマーカー
});
```

小さい値にすると目立ちにくいマーカーになり、細かいディテールのある画像に適しています。大きい値にすると、情報の多い画像やモバイルデバイスのユーザーにとって、マーカーが見やすくクリックしやすくなります。

#### hasDarkBackground

ページの背景が暗い場合に、ダークモードのスタイリングを有効にします。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

コメント数が変わるたびに呼び出されるコールバック関数です。バッジやページタイトルなどの UI 要素の更新に便利です。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### 継承される設定オプション

Image Chat は標準のコメントウィジェットを拡張しているため、ベースの FastComments ウィジェットの任意の設定オプションを使用できます。よく使われるオプションの例を以下に示します:

#### locale

ウィジェット UI の言語を設定します。FastComments は数十の言語をサポートしています。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // スペイン語
});
```

#### readonly

すべての会話を読み取り専用にします。ユーザーは既存のマーカーやディスカッションを閲覧できますが、新規作成や返信はできません。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso と simpleSSO

シングルサインオン(SSO)を使用して、お使いの認証システムと統合できます。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // SSO 設定
    }
});
```

認証オプションの詳細については、SSO ドキュメントをご覧ください。

#### maxReplyDepth

返信が何階層までネストできるかを制御します。デフォルトでは、Image Chat はこれを 0 に設定しており、すべてのコメントがフラット(ネストされた返信なし)になります。スレッド形式の会話が必要な場合は変更できます。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // 3 階層までのネストを許可
});
```

### 内部設定

これらのオプションは Image Chat によって自動的に設定されるため、上書きしないでください:

Image Chat では `productId` が自動的に `2` に設定されます。チャットウィンドウの機能を提供するために、`floating-chat` 拡張が自動的に読み込まれます。また、ウィジェットはモバイルデバイス(幅 768px 未満の画面)を自動検出し、フルスクリーンのチャットウィンドウを使用するように UI を調整します。

### 対象要素の柔軟性

`FastCommentsImageChat` の最初のパラメータには、`<img>` 要素を直接指定することも、画像を含むコンテナ要素を指定することもできます:

```javascript
// 画像要素を直接指定
FastCommentsImageChat(document.getElementById('my-image'), config);

// 画像を含むコンテナを指定
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

コンテナ要素を渡した場合、ウィジェットが自動的に画像を見つけます。

### 完全な例

複数の設定オプションを組み合わせた例を以下に示します:

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // お使いの SSO 設定
    },
    maxReplyDepth: 1
});
```

ベースウィジェットから継承されるすべての利用可能な設定オプションの一覧については、FastComments のメイン設定ドキュメントをご覧ください。