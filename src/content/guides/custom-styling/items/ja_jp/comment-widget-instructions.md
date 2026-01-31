## コメントウィジェットのスタイルをカスタマイズする方法

コメントウィジェットのスタイリングは、次の2つの方法でカスタマイズできます:

### オプション 1: `customCSS` パラメータで

ウィジェットを初期化するときに、カスタムCSSを文字列として `customCSS` パラメータに渡します:

```javascript
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: 'your-tenant-id',
    customCSS: `
        .fast-comments .comment {
            background-color: #f0f0f0 !important;
            border-radius: 8px !important;
        }
    `
}];
```

### オプション 2: 管理ダッシュボードから

1. 管理ダッシュボードの[ウィジェットのカスタマイズページ](https://fastcomments.com/auth/my-account/customize-widget)に移動
2. 「詳細」内の「カスタム CSS」セクションまでスクロール
3. カスタム CSS を入力
4. 「保存」をクリック

入力したカスタム CSS は、サイト上のすべてのコメントウィジェットに適用されます。

## ヒント

- 必要に応じてデフォルトのスタイルを上書きするために `!important` を使用してください
- サイトの他の部分に影響を与えないよう、特定のセレクタを指定してください
- 互換性のために、異なるブラウザで CSS をテストしてください
- ウィジェットは標準の CSS を使用します — 特別なプリプロセッサは不要です

---