## コメントウィジェットのスタイルをカスタマイズする方法

コメントウィジェットのスタイルは次の2つの方法でカスタマイズできます：

### オプション 1：customCSS パラメータを使用

ウィジェットを初期化するときに、`customCSS` パラメータにカスタムCSSを文字列として渡します：

```javascript
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: 'your-tenant-id',
    customCSS: `
        .fast-comments .comment {
            background-color: #f0f0f0 !important;
            border-radius: 8px !important;
        }
    `
});
```

### オプション 2：管理ダッシュボードから

1. 管理ダッシュボードの[ウィジェットのカスタマイズページ](https://fastcomments.com/auth/my-account/customize-widget)に移動します
2. 「Advanced」の下の「Custom CSS」セクションまでスクロールします
3. カスタムCSSを入力します
4. 「保存」をクリックします

入力したカスタムCSSはサイト上のすべてのコメントウィジェットに適用されます。

## ヒント

- 必要に応じてデフォルトのスタイルを上書きするために `!important` を使用してください
- サイトの他の部分に影響を与えないように特定のセレクタをターゲットにしてください
- 互換性のために、複数のブラウザで CSS をテストしてください
- ウィジェットは標準のCSSを使用します。特別なプリプロセッサは不要です