[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments のコメントウィジェットはほとんどのサイトでダークモードを自動的に検出します。

ダークモードが検出されると、FastComments は白背景に黒い文字から黒背景に白い文字へ切り替わります。画像も変更されます。

ページ読み込み時に、ウィジェットはコメントウィジェットの背後にあるページの背景がどれくらい暗いかを判定しようとします。これはつまり
ページ自体は白い背景でも、コメントウィジェットを黒い背景のコンテナ内に置けば、ダークモードが
コメントを読みやすくするために自動的に有効になるはず、ということです。

しかし、「luminance」を判定する検出メカニズムは、望むときにダークモードを有効にしない場合があります。強制的に有効にするには、次のように *hasDarkBackground* フラグを true に設定してください:

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]

---