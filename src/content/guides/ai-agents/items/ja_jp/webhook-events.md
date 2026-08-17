There are four agent webhook event types. Each event has a numeric enum value (used in payloads) and a canonical string name (used in the `event` envelope field and in the `X-FastComments-Agent-Event` HTTP header).

| Event name | Enum | Fires when |
|---|---|---|
| `trigger.succeeded` | 0 | エージェントの実行がステータス`SUCCESS`で完了したとき。 |
| `trigger.failed` | 1 | エージェントの実行がステータス`ERROR`で完了したとき。 |
| `approval.requested` | 2 | 承認が`PENDING`状態でキューに入れられたとき。 |
| `approval.decided` | 3 | 承認が`APPROVED`、`REJECTED`、または`EXECUTION_FAILED`に遷移したとき。 |

### `trigger.succeeded`

エージェントの実行がエラーなく完了した後に発火します。ペイロードの`data`フィールドには以下が含まれます：

- `triggerId` - ユニークな実行ID。
- `triggerType` - 実行を開始した[トリガー理由enum](#triggers-overview)。
- `status` - `SUCCESS`（文字列）。
- `tokensUsed` - この実行で消費されたトークン。
- `wasDryRun` - エージェントが[ドライランモード](#dry-run-mode)だった場合はtrue。
- `actions` - `TenantAgentAction`レコードの配列（[Webhook Payloads](#webhook-payloads)参照）。
- `commentId`, `url`, `urlId` - トリガーがそれらを持っている場合。

実行がゼロアクションであった場合、`actions`配列は空になります。これは「エージェントが何もしないことを決定めた」成功した実行であり、知っておくと便利です。

### `trigger.failed`

実行がエラーになると発火します。ペイロードの形は`trigger.succeeded`と同じで、`status: 'ERROR'`と、何が問題だったかを示す追加の`errorMessage`フィールドがあります。考えられるエラーには、LLM呼び出しの失敗、ツールのディスパッチ失敗、実行中の予算枯渇などがあります。

`actions`にはエラーが発生する前に完了したツール呼び出しのエントリが含まれる場合があります。

### `approval.requested`

`PENDING`状態で承認がキューに入れられた瞬間に発火します。ペイロードには以下が含まれます：

- `approvalId`, `triggerId`。
- `toolName`, `actionType`。
- `status: 'PENDING'`。
- `args` - ツールの引数で、LLM呼び出しから**そのまま**渡されます。形状はツールごとに異なり、安定した公開契約ではありません。新しいツールが追加されるとスキーマが変わる可能性があります。
- `createdAt`。
- `justification`, `confidence` - エージェントが提供した場合。
- `contextSnapshot` - 承認が関連するコメント/ページのコンテキスト。

保留中の承認をチャットOpsチャンネルに転送するのに便利です。`approval.requested`を購読したSlackボットは、アクションとその理由をモデレーションチャンネルに投稿し、一目でレビューできるようにします。

### `approval.decided`

`PENDING`から承認が移動したときに発火します。ペイロードには以下が含まれます：

- `approvalId`, `triggerId`。
- `toolName`, `actionType`。
- `status` - `APPROVED`、`REJECTED`、または`EXECUTION_FAILED`。
- `decidedBy` - 決定したモデレーターのユーザーID。
- `decidedAt` - 彼らが決定した時刻。
- `executedAt` - `APPROVED`の場合、プラットフォームが承認されたアクションを実行した時刻。
- `executionResult` - `APPROVED`の場合、実行者の結果を示す文字列。
- `contextSnapshot` - コメント/ページのコンテキスト。

このイベントはすべての決定結果をカバーします：

- **承認され、正常に実行** -> `status: APPROVED`、`executedAt`が設定、`executionResult`は成功メッセージ。
- **承認されたが実行者が失敗** -> `status: EXECUTION_FAILED`、`executedAt`が設定、`executionResult`は失敗を記述。
- **却下** -> `status: REJECTED`、`executedAt`はnull、`executionResult`はnull。

### Header

すべての配信には、イベントの正規文字列名（`trigger.succeeded`など）を含む`X-FastComments-Agent-Event` HTTPヘッダーが含まれます。エンドポイントが複数のイベントタイプを処理する単一のURLの場合に便利です。

### See also

- 各イベントのペイロードスキーマ全体については[Webhook Payloads](#webhook-payloads)をご覧ください。
- HMAC方式については[Webhook Signing](#webhook-signing)をご覧ください。
- 配信セマンティクスについては[Webhook Retries](#webhook-retries)をご覧ください。