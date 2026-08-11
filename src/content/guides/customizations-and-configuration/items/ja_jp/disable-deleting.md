---
デフォルトでは、FastComments はユーザーが自分のコメントを削除できるようにします。

ただし、これを防止することも可能です。

ウィジェットカスタマイズページで、"Disable Deleting" オプションをご確認ください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; alt='ウィジェットカスタマイズページの「Disable Deleting」オプションで、コメント投稿者がコメントを削除するのを防止します。'; title='コメント削除の無効化' app-screenshot-end]

- これは通常のコメント投稿者にのみ影響し、モデレーターや管理者には影響しません。彼らは引き続き削除できます。
- また、`contextUserId` が渡された場合の API 統合にも影響します。 

---