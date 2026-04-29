The **Community guidelines** フィールドは、編集フォーム上の任意のポリシー用テキストブロックであり、このエージェントの各実行時に user-role コンテキストメッセージに含まれます。これは信頼されないテキストとしてフェンスで囲まれており（プラットフォームがコメント本文や他のユーザー提供コンテンツに適用するのと同じフェンシング）、モデルはこれをシステム指示ではなくポリシー参照として扱います。ここは「このサイトでどのような行為が許容され、許容されないか」を記述する正準的な場所であり、エージェントが一貫して適用できるようにします。

### How it differs from the initial prompt

- **Initial prompt** - エージェントの役割と意思決定スタイル。 "You are a moderator. Prefer warning over banning."
- **Community guidelines** - コミュニティのルールをポリシー言語で表現したもの。 "No personal attacks. No promotional links from accounts under 24 hours old. Off-topic comments may be removed if a thread is heated."

両者は同じコンテキストウィンドウに流れ込みますが、異なるレイヤーで入力されます - initial prompt は system role の一部であり、guidelines ドキュメントは user-role コンテキストメッセージ内のフェンスで囲まれたテキストです。この分離により、どちらか一方を更新したいときにもう一方を読み直す必要がなく、編集が容易になります。

### What's a good guidelines doc

短く、具体的で、人間が書いた文書。箇条書きは散文よりも有効です:

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

エージェントはこれを各実行で適用します。ガイドラインを変更した場合、その変更は次にトリガーされた実行から有効になります — 過去の実行に対して遡及的に再評価されることはありません。

### What not to put here

- **Output formatting instructions** ("reply in HTML", "use emoji"). これらは[initial prompt](#personality-prompt)に属します。
- **Localized text.** ガイドライン文書は、プロンプトと同様に**英語のみ**であるべきです。機械翻訳はエージェントの動作を目に見えない形で変えてしまう可能性があるためです。もしロケールごとに異なるポリシーがある場合は、この一つの文書にすべて英語で書き、ドキュメントを例えば "for German-language pages: ..." のように構成してください。
- **Long quotations of external policies.** 要約してください。長い引用は各実行でトークンコストがかさみます。
- **PII or secrets.** このテキストは各実行で LLM プロバイダに送信されます。

### Length

このフィールドは **4000 characters** に制限されています（フォームと保存ルートの両方で強制）。各実行時のトークンコストは長さに比例するため、上限内でも通常は数百語で十分です。コミュニティポリシーが多数ページに及ぶ場合は、このフィールドにエージェントが必要とする部分を要約して含めてください。

### Versioning

ガイドライン文書には組み込みのバージョン履歴はありません — 最新の保存値がエージェントによって使用されます。履歴が必要な場合は、主要な編集のたびに自分のトラッキングシステムにドキュメントをコピーしてください。[Refine Prompts](#refining-prompts) のフローは *initial prompt* の変更を記録できますが、ガイドライン文書自体をバージョン管理するものではありません。

---