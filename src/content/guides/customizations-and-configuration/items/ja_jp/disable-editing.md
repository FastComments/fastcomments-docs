---
デフォルトでは、FastComments はユーザーが自分のコメントを編集できるようにします。

ただし、これを防止することも可能です。

ウィジェットカスタマイズページで、「編集の無効化」オプションをご確認ください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; alt='ウィジェットカスタマイズページの編集無効化オプションで、コメント投稿者がコメントを編集できないようにします'; title='コメント編集の無効化' app-screenshot-end]

- これは通常のコメント投稿者にのみ影響し、モデレーターや管理者には影響しません。彼らは引き続き編集可能です。
- `contextUserId` が渡された場合の API 統合にも影響します。 

---