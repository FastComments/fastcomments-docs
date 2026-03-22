Recent Discussions ウィジェットは、サイト上で最新のコメント活動があったページを表示します。各項目にはページタイトル、最終アクティビティの日付、およびコメント合計数が表示されます。ダークな背景を自動検出し、それに応じてスタイルを調整します。

## 基本インストール

[inline-code-attrs-start title = 'Recent Discussions ウィジェットのインストール'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## 設定オプション

`FastCommentsRecentDiscussionsV2` 関数は次の設定オプションを受け取ります:

- **tenantId** (required): あなたの FastComments テナント ID
- **count** (optional): 表示するページ数。デフォルトは `20`、最大 `100`
- **hasDarkBackground** (optional): ダークモードのスタイリングを強制します。未設定の場合はページの背景から自動検出されます

## 応用例

### カスタムカウント

[inline-code-attrs-start title = 'カスタムカウントのRecent Discussions'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        count: 5
    });
</script>
[inline-code-end]

### ダークモードを強制

[inline-code-attrs-start title = 'ダークモードのRecent Discussions'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

---