編集フォームの **Community guidelines** フィールドは、このエージェントの実行ごとにユーザーロールのコンテキストメッセージに含まれる、任意のポリシーテキストブロックです。コメント本文やその他のユーザー提供コンテンツに対してプラットフォームが適用するのと同じフェンシングにより、信頼できないテキストとして囲まれるため、モデルはこれをシステム指示ではなくポリシーの参照情報として扱います。「このサイトで許可される行動と許可されない行動」を書き記し、エージェントが一貫して適用できるようにするための正規の場所です。

### 初期プロンプトとの違い

- **初期プロンプト** - エージェントの役割と判断スタイル。「あなたはモデレーターです。BANよりも警告を優先してください。」
- **Community guidelines** - ポリシー言語で書かれたコミュニティのルール。「個人攻撃は禁止。24時間未満のアカウントによる宣伝リンクは禁止。スレッドが白熱している場合は、脱線コメントを削除することがあります。」

両方とも同じコンテキストウィンドウに流れ込みますが、入るレイヤーが異なります。初期プロンプトはシステムロールの一部であり、ガイドライン文書はユーザーロールのコンテキストメッセージ内のフェンスされたテキストです。この分離により、片方だけを更新したい場合に、もう片方を読み返さずに済むため、編集が容易になります。

### 良いガイドライン文書とは

短く、具体的で、人間が書いた文書です。散文よりもリスト形式が効果的です:

[inline-code-attrs-start title = 'コミュニティガイドラインの例'; type='text' inline-code-attrs-end]
[inline-code-start]
Allowed:
- Substantive disagreement, even strongly worded.
- Links to original sources, even from new accounts.
- Off-topic asides if the parent thread permits them.

Not allowed:
- Personal attacks against specific named users.
- Doxxing or sharing of private information.
- Coordinated promotional activity (multiple comments pushing the same external link).
- Comments that exist only to derail discussion.

Borderline:
- Strong language without a target. Allowed if not directed at a person.
- Political topics outside the page subject. Off-topic; warn first, don't remove unless persistent.
[inline-code-end]

エージェントは毎回の実行でこれを適用します。ガイドラインを変更した場合、その変更は次回のトリガー時に有効になります。過去の実行が遡って再評価されることはありません。

### ここに書くべきでないもの

- **出力フォーマットの指示**(「HTMLで返信する」「絵文字を使う」など)。これらは[初期プロンプト](#personality-prompt)に属します。
- **ローカライズされたテキスト。** ガイドライン文書は、プロンプトと同様に、同じ理由により**英語のみ**である必要があります。機械翻訳はエージェントの動作を黙って変えてしまう可能性があります。ロケールによって異なるポリシーがある場合は、すべて英語でこの1つの文書に記述し、「ドイツ語ページの場合: ...」のように文書を構成してください。
- **外部ポリシーの長い引用。** 要約してください。長いコンテキストは毎回の実行でトークンを消費します。
- **PIIやシークレット。** このテキストは毎回の実行でLLMプロバイダーに送信されます。

### 長さ

このフィールドは**4000文字**で上限が設定されています(フォームと保存ルートの両方で強制されます)。毎回の実行でのトークンコストは長さに比例するため、上限内であっても数百語で通常は十分です。コミュニティのポリシーが何ページにもわたる場合は、エージェントが必要とする部分をこのフィールド専用の要約にまとめてください。

### バージョン管理

ガイドライン文書には組み込みのバージョン履歴はありません。エージェントが使用するのは最新の保存値です。履歴が必要な場合は、大きな編集を行う前に、文書を独自の管理システムにコピーしてください。[プロンプトの改善](#refining-prompts)フローでは*初期プロンプト*への変更を記録できますが、ガイドライン文書のバージョン管理は行われません。