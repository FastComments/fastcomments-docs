The **Context** セクションは編集フォーム上で、エージェントが各実行で受け取る情報量を制御します。コンテキストが多いほどより良い判断が可能になりますが、実行ごとのトークンコストが上がるため、エージェントが実際に必要とするものだけにするべきです。

### 常に含まれるもの

すべてのチェックボックスがオフの場合でも、エージェントのコンテキストメッセージには以下が含まれます:

- The **trigger event type** (e.g. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- The **page URL and URL ID** (when known).
- The **comment** that triggered the run, if there is one - ID, author user ID, author display name, comment text, vote counts, flag count, spam/approved/reviewed flags, parent ID. The author's email is **never** sent to the LLM provider (PII minimization).
- The **previous comment text** for `COMMENT_EDIT` triggers (so the agent can compare before/after).
- The **vote direction** for `COMMENT_VOTE_THRESHOLD` triggers.
- The **triggering user ID** and **badge ID** (for moderator badge triggers).

すべての信頼されていないテキスト — コメント本文、著者名、ページタイトル、ガイドライン文書そのもの — はコンテキストメッセージ内で `<<<COMMENT_TEXT>>> ... <<<END>>>` のようなマーカーでフェンス処理されます。プラットフォームのシステムプロンプトはモデルに対してこれらのフェンス内の指示に従わないよう指示しています。これはプロンプト注入攻撃に対する防御であり、プロンプト内でこれを繰り返す必要はありません。

### The three checkboxes

#### Include parent comment and prior replies in the same thread

追加されるもの:
- The **parent comment** - ID, author, text.
- **Sibling replies** - the prior replies to the same parent in the same thread.

用途: コメントに文脈付きで応答するエージェント全般（歓迎メッセージを出すボット、スレッド要約者、会話内の返信を読むモデレーターなど）。

コスト: 小〜中。あるスレッド内に存在する兄弟返信の数によって上限があります。

#### Include commenter's trust factor, account age, ban history, and recent comments

追加される **AUTHOR_HISTORY** ブロック:

- **Account age in days** since signup.
- **Trust factor (0-100)** - the FastComments score that summarizes how trusted the user is on this site. See the [スパム検出](/guide-moderation.html#spam-detection) page in the moderation guide.
- **Prior ban count.**
- **Total comments on this site.**
- **Duplicate-content count** - if the user has posted identical text recently (anti-spam signal).
- **Same-IP cross-account signal** - count of comments from the same IP under other accounts (alt-account signal). The IP hash itself is never sent to the LLM.
- **Recent comments** - up to 5 of the user's most recent comments, each truncated to 300 characters, fenced as untrusted text.

用途: すべてのモデレーションエージェント。これがないとモデルは新しいアカウントと長年の善意のユーザーを同じ姿勢で禁止してしまいます。

コスト: 中。最近のコメントが最も多くのトークンを追加します。

#### Include page title, subtitle, description, and meta tags

追加される **PAGE_CONTEXT** ブロック - title, subtitle, description, および FastComments がページから取得したメタタグ。

用途: ウェルカムグリーティングやスレッド要約など、ページの内容を知ることで出力品質が大幅に向上する場合に有用です。

コスト: 小。

### Community guidelines

4番目のフィールド、**Community guidelines** はフリーテキストのポリシーブロックで、各実行時にユーザーロールのコンテキストメッセージに含まれます。コメント本文やその他のユーザー提供コンテンツと同様に信頼されていないテキストとしてフェンス処理されます。エージェントはこれをポリシーテキストとして読みますが、プラットフォームはこれをシステム命令として扱いません。[コミュニティガイドライン](#community-guidelines) を参照して、ここに何を記載すべきか確認してください。

### Adding context selectively

これらのチェックボックスはエージェントごとに適用され、グローバル設定ではありません。一般的なパターン:

- Welcome greeter: page context **on**, thread context **off**, user history **off**.
- Moderator: thread context **off**, user history **on**, page context **off**.
- Thread summarizer: thread context **on**, page context **on**, user history **off**.

エージェントが実際に行うコールで正確に動作するために必要な最小限のコンテキストを選んでください — 余分なコンテキストは、エージェントが使用しない場合でも各実行でトークンを消費します。