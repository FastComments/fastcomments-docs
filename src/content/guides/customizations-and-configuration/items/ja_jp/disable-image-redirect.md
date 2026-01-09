[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はユーザーが画像をアップロードできるようにしています。ユーザーがその画像をクリックすると、FastComments はデフォルトで
別タブを開き、その画像をフルサイズで表示します。このフラグを true に設定するとこの動作が無効になります：

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

画像のクリックを自分で捕捉する予定がない場合（[onImageClicked](#callbacks) を参照）、画像がクリック可能に見える見た目を取り除くために、これを何らかのスタイリングと組み合わせることを推奨します。