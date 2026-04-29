Analytics はエージェント横断ダッシュボードです。AI Agents ページの **Analytics** タブ（テナント全体）から、または各エージェント行の **Analytics** ボタンからエージェント単位でアクセスできます。

### Filter

上部のドロップダウン - **All agents** または特定のエージェント。ページの残りはそれに応じてスコープが変更されます。

### Budget usage

現在期間の支出を上限と比較して示す4つのプログレスバー：

- **Agent today**（特定のエージェントにフィルタしたとき） - エージェントの日次上限。
- **Agent this month** - エージェントの月次上限。
- **Account today** - テナントの日次上限。
- **Account this month** - テナントの月次上限。

上限が未設定の場合、バーには "(no cap set)" と表示され、生の支出が表示されます。

### Daily cost (last 30 days)

選択したスコープにおける、テナントの通貨での1日ごとのコストの表。以下を見つけるのに便利です：

- **Sudden cost spikes** - 通常は暴走ループやバイラルなコメントがトリガーを広げたことによるもの。
- **Cost drift** - コミュニティの成長に伴う徐々に増加する日次コスト。

### Actions taken

今月のアクション種類ごとの内訳 - 「Wrote a comment: 47」「Marked a comment as spam: 12」など。エージェントが期待通りに動作しているか確認するのに有用です。

### Triggers skipped (this month)

[drop reason](#drop-reasons) ごとに集計されたカウント：

- エージェントの日次 / エージェントの月次 / アカウントの日次 / アカウントの月次を超過。
- レート制限。
- 同時実行が飽和。

ここでドロップが見られる場合、エージェントは予算またはレート制限に達しており、通常は実行されるはずのトリガーを見逃しています。See [Drop Reasons](#drop-reasons)。

### Dry-run vs live (this month)

- **Enabled runs** - 今月実際のアクションを行った実行の数。
- **Dry runs** - 今月のドライランモードでの実行の数。

有用なチューニング信号：まだ Enabled に昇格していない新しいエージェントはドライランのみが表示されます。Enabled の状態でこの欄のすべてのカウントがゼロの場合はアイドル状態です — トリガーされていないか、スコープから除外されているか、トリガーの設定が正しくない可能性があります。

### Top agents by monthly cost

フィルターが **All agents** のとき、ページは月次累計コストでランク付けされたエージェントを一覧表示します。最も高コストのエージェントを見つけることはコスト最適化の第一歩です — 通常の対処は「[context options](#context-options) を絞る」か「[budget cap](#budgets-overview) を下げる」です。

### Agents at or near their cap

現在期間においてエージェントごとの、エージェント上限に達しているか近いエージェントの内訳：

- **near cap** - 上限の設定された割合を超えている。
- **over cap** - 実際に上限に達しており、その期間に `{count} dropped` のトリガーが発生している。

この表からエージェントをクリックして、上限を引き上げる、スコープを狭める、または一時停止することができます。

### Account summary

フィルターが **All agents** のとき：

- **Triggers today** - 件数。
- **Triggers this month** - 件数。
- それぞれについて：スキップされた数を示す `dropped` サフィックス。

### Currency

すべての金額はテナントの通貨で表示されます。

### What this page does not do

- **per-action cost breakdowns** は表示しません - それらは [Run Detail View](#run-detail-view) にあります。
- **transcripts** や **LLM responses** は表示しません。
- エージェントに対する操作はこのページからは行えません - 編集、停止、削除はすべてエージェント一覧 / 編集ページから行います。