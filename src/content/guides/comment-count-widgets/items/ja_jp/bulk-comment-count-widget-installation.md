バルクコメントカウントウィジェットは、同じページで複数のページのコメント数を効率的に表示するために設計されています。各コメント数に対して個別のAPI呼び出しを行う代わりに、このウィジェットはリクエストをバッチ処理して最適なパフォーマンスを実現します。

## 基本インストール

[inline-code-attrs-start title = 'Bulk Comment Count Widget Installation'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<!-- Multiple elements with comment counts -->
<div class="fast-comments-count" data-fast-comments-url-id="page-1"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-2"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-3"></div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## 動作の仕組み

バルクウィジェットは以下のように動作します：

1. `fast-comments-count`クラスを持つ要素をページでスキャン
2. 各要素から`data-fast-comments-url-id`属性を読み取り
3. 複数のコメント数を効率的に取得するためにAPIリクエストをバッチ処理
4. 各要素を適切なコメント数で更新

## 設定オプション

`FastCommentsCommentCountBulk`関数は以下の設定オプションを受け入れます：

- **tenantId**（必須）：FastCommentsのテナントID
- **apiHost**（オプション）：セルフホストインスタンスを使用している場合のカスタムAPIホスト

## 実世界の例

ブログ投稿リストでバルクウィジェットを使用する方法を示す実用的な例：

[inline-code-attrs-start title = 'Blog Post Listing with Comment Counts'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .blog-post {
        border: 1px solid #ddd;
        margin: 10px 0;
        padding: 15px;
        border-radius: 5px;
    }
    .post-meta {
        color: #666;
        font-size: 14px;
        margin-top: 10px;
    }
    .comment-count {
        background: #e7f3ff;
        padding: 2px 8px;
        border-radius: 12px;
        font-size: 12px;
        display: inline-block;
    }
</style>

<div class="blog-post">
    <h3>How to Install FastComments</h3>
    <p>Learn how to add FastComments to your website in just a few minutes...</p>
    <div class="post-meta">
        Published: March 15, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="how-to-install-fastcomments"></span>
    </div>
</div>

<div class="blog-post">
    <h3>Advanced FastComments Configuration</h3>
    <p>Dive deep into the advanced configuration options for FastComments...</p>
    <div class="post-meta">
        Published: March 10, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="advanced-fastcomments-config"></span>
    </div>
</div>

<div class="blog-post">
    <h3>FastComments vs Other Solutions</h3>
    <p>See how FastComments compares to other commenting solutions...</p>
    <div class="post-meta">
        Published: March 5, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="fastcomments-comparison"></span>
    </div>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## パフォーマンスに関する考慮事項

バルクウィジェットは以下の方法でパフォーマンスを自動的に最適化します：

- **リクエストのバッチ処理**：複数のコメント数が単一のAPI呼び出しで取得される
- **リクエストサイズ制限**：URLリストが大きくなりすぎると（1,000文字以上）、リクエストは自動的に分割される
- **重複排除**：同じ`data-fast-comments-url-id`を持つ複数の要素は同じカウントを共有

## 同じURL IDを持つ複数の要素

同じ`data-fast-comments-url-id`を持つ複数の要素をページに配置できます。それらはすべて同じカウントで更新されます：

[inline-code-attrs-start title = 'Multiple Elements Same URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .count-example {
        margin: 10px 0;
        padding: 10px;
        background: #f9f9f9;
        border-radius: 5px;
    }
</style>

<div class="count-example">
    Header Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Sidebar Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Footer Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## ローカライゼーション

バルクウィジェットはFastCommentsの言語設定に基づいてコメント数を自動的にフォーマットします。以下に適切なテキストを提供します：

- コメントなし
- 1件のコメント
- 複数のコメント

## バルクウィジェットとシングルウィジェットの使い分け

**バルクウィジェットを使用する場合：**
- 同じページに複数のコメント数がある
- コメント数付きの投稿/記事リストを表示している
- パフォーマンスが重要（API呼び出しを削減）

**シングルウィジェットを使用する場合：**
- ページに1つのコメント数だけが必要
- ライブ更新が必要（シングルウィジェットはリアルタイム更新をサポート）
- 個々のウィジェットの動作をより細かく制御したい
