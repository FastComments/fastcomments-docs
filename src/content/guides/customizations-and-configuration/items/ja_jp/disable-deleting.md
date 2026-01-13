---
デフォルトでは、FastComments はユーザーが自分のコメントを削除できるようにします。

ただし、この動作を無効にすることが可能です。

ウィジェットのカスタマイズページで「削除を無効にする」オプションを参照してください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; title='Disable Comment Deleting' app-screenshot-end]

- これは通常のコメント投稿者にのみ影響し、モデレーターや管理者は引き続き削除できます。
- `contextUserId` が渡される API 統合にも影響します。 

---