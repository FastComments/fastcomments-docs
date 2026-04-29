すべてのエージェントには利用上限（spend caps）が設定されています。上限に達するとプラットフォームはエージェントのディスパッチを停止し、期間が切り替わると再開します。

### Two scopes, two periods

合計で4つのキャップがあります — 2つのスコープ（エージェント単位、テナント単位）と2つの期間（日次、月次）の組み合わせです。

| Scope | Period | Where you set it |
|---|---|---|
| Per-agent daily | UTC day | Agent edit form -> **Budget** -> **Daily budget** |
| Per-agent monthly | calendar month | Agent edit form -> **Budget** -> **Monthly budget** |
| Per-tenant daily | UTC day | Plan-derived (no separate user-facing input) |
| Per-tenant monthly | calendar month | Plan-derived (no separate user-facing input) |

トリガーは**4つすべてのキャップ**が許可する場合にのみディスパッチされます。最初に枯渇したキャップがトリガーを落とす原因になります。

### Currency

エージェントごとの予算はアカウントの通貨で入力します。

### What happens when a cap is reached

- トリガーは `agentDaily` や `tenantMonthly` のような [drop reason](#drop-reasons) として **dropped** と記録されます。
- ドロップした件数は [Analytics page](#analytics-page) の「Triggers skipped (this month)」に表示されます。
- LLM コールは行われません；ドロップしたトリガー自体に対してトークンは消費されません。
- エージェントのステータスは変更されません — 期間が切り替わるまで単にディスパッチできなくなります。

### Period roll-over

- **Daily** キャップはUTCの真夜中にリセットされます。
- **Monthly** キャップは各暦月の開始時（UTC）にリセットされます。

未使用の予算が次の期間に繰り越されることはありません。

### Hard cap vs soft warnings

キャップは**ハード**です。「10%超過して警告する」といったモードはありません。キャップに達するとディスパッチは停止します。

「ソフト」な部分は [Budget Alerts](#budget-alerts) のメールです — デフォルトで閾値80%と100%など、設定可能な閾値でメールを受け取れるため、トラフィックが落ち始める前にキャップを引き上げることができます。

### Where to read current usage

- [Analytics page](#analytics-page) - エージェント別およびテナント全体の予算使用量とキャップのマーカー。
- エージェント編集フォームの **Stats** セクション。
- 一覧ビュー（保留承認数や最近の実行数はエージェントカードに表示されます）。

### Picking a budget

いくつかの目安：

- **新しいエージェント** - まずは予算を決めます。[Run History](#run-history) を1週間観察してください。1実行あたりの実際のコスト × 期待されるトリガー量に基づいて調整します。
- **高トラフィックのエージェント**（例：忙しいサイトの新規コメントトリガー） - 日次キャップが暴走を食い止めます。通常の忙しい日が余裕をもって収まるように、期待される日次支出の2〜3倍のデイリーキャップを選んでください。
- **要約やコンテキスト重視のエージェント** - 1実行あたりのコストが高くなりがちです。月間を台無しにしないよう、より厳しい日次キャップを設定してください。

### Budget bypass for replays

[Test runs / replays](#test-runs-replays) にはそれ自体のハードキャップがあります（リプレイフォームで設定、エージェントのデイリー／月次キャップとは別）。さらにエージェントおよびテナントのキャップも適用されます。先に到達した方がリプレイを停止します。

### See also

- [Budget Alerts](#budget-alerts) — メール通知について。
- [Cost Model](#cost-model) — プラットフォームがトークンをドルに変換する方法。
- [Drop Reasons](#drop-reasons) — トリガーが実行されない理由の完全な一覧。