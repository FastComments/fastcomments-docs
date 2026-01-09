これからカスタムの FastComments コードを生成します。以下のウィザードを使って、FastComments を GoHighLevel サイトでどのように動作させるかを設定してください:

[snippet id="gohighlevel-wizard"]

### 異なるコメントボックスの種類

`TYPE = 'commenting'` の行を設定して使用するプロダクトを切り替えることができます（例えばストリーミングチャットには `live` に、コラボチャットには `collab` に変更できます）。

### コメントボックスを任意の場所に配置する

ページの特定の部分にコメントボックスを配置し、デフォルトの場所ではなくしたいとします。
この行を変更してください:

    const TARGET_ELEMENT_ID = ''; // set to use target div mode

に:

    const TARGET_ELEMENT_ID = 'fc_box'; // set to use target div mode

それから GHL エディタで「code」ボタンをクリックし、コメントを表示したい場所に追加してください:

[inline-code-attrs-start title = 'GoHighLevel FastComments の Div'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### ページごとに異なるコメントボックスタイプ

ユーザーにテキストの一部をハイライトして議論させたり、代わりにストリーミングチャットの UI を使用させたいとします。

まずは上記の「コメントボックスを任意の場所に配置する」の手順に従ってください。

その小さなスニペット内には `type="commenting"` とあります。

例えば collab チャットを有効にしたい場合は `type="collab"` に変更してください。

### 特定のページにのみ表示する

`TARGET_ELEMENT_ID` を設定しない場合、代わりに `VALID_PATTERNS` 変数を設定して、コメントを表示する URL ルートを指定することができます。デフォルトでは、URL に `/post` が含まれるページに表示されます。

### コラボチャットの設定

コラボチャットに、特定のエリア内の HTML 周辺だけに共同機能を追加するよう指示することができます。例えば、上記のフッターコードを追加し、その後投稿/ページのコンテンツ内に次の div を追加してコラボチャットを有効にするとします:

[inline-code-attrs-start title = '指定コンテンツでのコラボチャット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

すると `<div>` 内の paragraph 要素にのみコラボチャットが有効になり、ページの他の部分には影響しません。`<div>` に何もコンテンツを入れない場合は、投稿本文全体でコラボチャットが有効になります。

---