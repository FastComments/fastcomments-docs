### 概要

FastComments Image Chatは標準のFastCommentsコメントウィジェットを拡張しており、ベースウィジェットのすべての設定オプションを継承しつつ、画像注釈に特化したいくつかのオプションを追加します。

### 必須の設定

#### tenantId

FastCommentsのTenant IDが必要です。これは[FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret)で確認できます。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat 固有のオプション

#### urlId

既定では、Image ChatはページのURL、画像のソース、X/Y座標に基づいて各会話のための一意の識別子を生成します。カスタムの`urlId`でこれを上書きできます。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

これは、URL構造が変わる可能性があるが同じ会話を保持したい場合や、複数のページにわたって注釈を共有したい場合に便利です。

#### chatSquarePercentage

クリック可能なチャットマーカーのサイズを画像幅に対するパーセンテージで制御します。デフォルトは5%で、各マーカーは画像幅の5%になります。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // より大きく、目立つマーカー
});
```

小さい値は詳細な画像に対して目立ちにくいマーカーを作ります。大きい値は、情報量の多い画像やモバイルデバイスのユーザーにとってマーカーを見やすく、クリックしやすくします。

#### hasDarkBackground

ページにダークな背景がある場合にダークモードのスタイリングを有効にします。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

コメント数が変化するたびに発火するコールバック関数です。バッジやページタイトルなどのUI要素を更新するのに便利です。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### 継承された設定オプション

Image Chatは標準のコメントウィジェットを拡張しているため、ベースのFastCommentsウィジェットの任意の設定オプションを使用できます。以下はよく使われるオプションです：

#### locale

ウィジェットUIの言語を設定します。FastCommentsは数十の言語をサポートしています。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
```

#### readonly

すべての会話を読み取り専用にします。ユーザーは既存のマーカーや議論を閲覧できますが、新しいマーカーの作成や返信はできません。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

シングルサインオンを使用して認証システムと統合します。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // SSOの設定
    }
});
```

認証オプションの詳細についてはSSOドキュメントを参照してください。

#### maxReplyDepth

返信の深さ（ネストのレベル）を制御します。既定では、Image Chatはこれを0に設定しており、すべてのコメントはフラット（ネストなし）です。スレッド化された会話にしたい場合はこれを変更できます。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Allow 3 levels of nesting
});
```

### 内部設定

これらのオプションはImage Chatによって自動的に設定され、上書きすべきではありません：

`productId`はImage Chatでは自動的に`2`に設定されます。チャットウィンドウ機能を提供するために`floating-chat`拡張が自動的に読み込まれます。ウィジェットは自動的にモバイルデバイス（幅768px未満の画面）を検出し、フルスクリーンのチャットウィンドウでUIを調整します。

### ターゲット要素の柔軟性

`FastCommentsImageChat`の最初のパラメータは、直接の`<img>`要素でも、画像を含むコンテナ要素でもかまいません：

```javascript
// 直接の画像要素
FastCommentsImageChat(document.getElementById('my-image'), config);

// 画像を含むコンテナ
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

コンテナ要素を渡した場合、ウィジェットは自動的に画像を見つけます。

### 完全な例

複数の設定オプションを組み合わせた例を示します：

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
        // あなたのSSO設定
    },
    maxReplyDepth: 1
});
```

利用可能なすべての設定オプションの完全なリストについては、ベースウィジェットから継承された設定に関するメインのFastComments構成ドキュメントを参照してください。