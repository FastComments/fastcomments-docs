最近のコメントウィジェットは、サイト全体または特定のページの最新コメントのリストを表示します。見出し、丸型アバター、コメントのプレビュー、コメントへのディープリンクとなるクリック可能な日付、そして自動ダークモード検出が含まれます。

## 基本的なインストール

[inline-code-attrs-start title = '最近のコメントウィジェットのインストール'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-comments-v2.min.js"></script>
<div id="fastcomments-widget-recent-comments"></div>
<script>
    FastCommentsRecentCommentsV2(document.getElementById('fastcomments-widget-recent-comments'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## 設定オプション

- **tenantId** (必須): あなたのFastCommentsテナントID
- **urlId** (任意): 単一のページに絞り込みます。すべてのページを対象とする場合はnullのままにしてください
- **count** (任意): 表示するコメント数。デフォルトは `10` です
- **hasDarkBackground** (任意): ダークモードのスタイリングを強制します。設定しない場合はページの背景から自動検出されます

## ウィジェットの構造

ウィジェットは以下のHTML構造でレンダリングされます:

[inline-code-attrs-start title = '最近のコメントウィジェットのHTML構造'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div class="fc-rc2">
    <div class="fc-rc2-heading">Recent Comments</div>
    <div class="fc-rc2-list">
        <div class="fc-rc2-card">
            <div class="fc-rc2-header">
                <img class="fc-rc2-avatar" src="..." alt="Avatar" />
                <div class="fc-rc2-meta">
                    <span class="fc-rc2-name">Username</span>
                    <a class="fc-rc2-date" href="...">2 hours ago</a>
                </div>
            </div>
            <div class="fc-rc2-body">Comment preview...</div>
            <a class="fc-rc2-page-link" href="...">Page Title</a>
        </div>
    </div>
</div>
[inline-code-end]

## デフォルトCSSリファレンス

[inline-code-attrs-start title = '最近のコメントウィジェットのデフォルトCSS'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rc2 {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Helvetica Neue", sans-serif;
    text-align: left;
    line-height: 1.5;
    color: #1a1a1a;
    border: 1px solid #e0e0e0;
    border-radius: 12px;
    padding: 20px;
    background: #fff;
}
.fc-rc2-heading { font-size: 16px; font-weight: 700; margin-bottom: 14px; padding-bottom: 12px; border-bottom: 1px solid #eee; }
.fc-rc2-card { padding: 14px 0; border-bottom: 1px solid #f0f0f0; }
.fc-rc2-card:last-child { border-bottom: none; }
.fc-rc2-header { display: flex; align-items: center; gap: 10px; margin-bottom: 8px; }
.fc-rc2-avatar { width: 32px; height: 32px; border-radius: 50%; object-fit: cover; }
.fc-rc2-name { font-size: 13px; font-weight: 600; }
.fc-rc2-date { font-size: 11.5px; color: #999; text-decoration: none; }
.fc-rc2-body { font-size: 14px; line-height: 1.55; color: #444; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; }
.fc-rc2-page-link { display: inline-block; margin-top: 6px; font-size: 12px; color: #777; text-decoration: none; }
.fc-rc2-page-link:hover { color: #0066cc; text-decoration: underline; }
[inline-code-end]

## カスタマイズ例

### アバターサイズの変更

[inline-code-attrs-start title = 'カスタムアバターサイズ'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rc2-avatar {
    width: 40px !important;
    height: 40px !important;
}
[inline-code-end]

### コメントテキストの表示行数を増やす

[inline-code-attrs-start title = 'コメント表示行数を増やす'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rc2-body {
    -webkit-line-clamp: 5 !important;
}
[inline-code-end]

### コンテナの枠線を削除する

[inline-code-attrs-start title = 'コンテナの枠線を削除'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rc2 {
    border: none !important;
    box-shadow: none !important;
}
[inline-code-end]