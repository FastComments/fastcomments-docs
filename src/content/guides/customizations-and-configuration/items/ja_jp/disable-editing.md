---
デフォルトでは、FastComments はユーザーが自分のコメントを編集できるようになっています。

ただし、これを無効にすることが可能です。

ウィジェットのカスタマイズページで「編集を無効化」オプションを参照してください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; title='Disable Comment Editing' app-screenshot-end]

- これは通常のコメント投稿者にのみ影響し、モデレーターや管理者には影響しません。彼らは引き続き編集できます。
- これは `contextUserId` が渡された場合の API 統合にも影響します。 

---