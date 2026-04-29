**Template ID:** `top_comment_pinner`

The Top Comment Pinner watches for top-level comments that cross a vote threshold and pins them - replacing whatever was pinned previously on the same thread.

The built-in prompt instructs the agent to skip replies (pinning works on threads, so pinning a reply is rarely useful) and to filter out promotional content (so the agent does not boost popular link-spam).

### Triggers

- **A comment crosses a vote threshold** (`COMMENT_VOTE_THRESHOLD`, default vote threshold: 10).

The trigger fires when the comment's net votes (`up - down`) reaches the configured threshold. Tune the number on the edit form based on how active your threads are - 10 is a sensible default for moderately active sites.

### Allowed tools

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Pinning is non-destructive - it can be reversed instantly - so this template usually runs without approvals.

### Recommended additions before going live

- **[Context Options](#context-options)で「親コメントと同じスレッド内の以前の返信を含める」にチェックを入れてください。** スレッドのコンテキストがないとエージェントは既にピンされているコメントをアンピンすべきかどうかを確実に判断できません。
- **投票閾値を調整する** — サイトに合わせて調整してください。活発なスレッドでは10は頻繁に起こりすぎますし、静かなスレッドでは10は一度も達成されないかもしれません。
- **URLでスコープを限定することを検討してください** — サイトの特定のセクション（例えばニューススレッド）だけにピンを適用し、告知スレッドには適用しない、など。

### Note on duplicate pinning

The agent's prompt instructs it to unpin first before pinning, but if the model misses that step the platform itself does not enforce a one-pinned-per-thread rule (you can have multiple). If duplicate pinning is a problem on your site, gate `pin_comment` behind approval and review each one - or write a tighter prompt.