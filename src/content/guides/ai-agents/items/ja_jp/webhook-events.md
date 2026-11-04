エージェントのWebhookイベントタイプは4種類あります。各イベントには、数値のenum値(ペイロードで使用)と正規の文字列名(`event`エンベロープフィールドおよび`X-FastComments-Agent-Event` HTTPヘッダーで使用)があります。

| イベント名 | Enum | 発火タイミング |
|---|---|---|
| `trigger.succeeded` | 0 | エージェントの実行がステータス`SUCCESS`で完了したとき。 |
| `trigger.failed` | 1 | エージェントの実行がステータス`ERROR`で完了したとき。 |
| `approval.requested` | 2 | 承認が`PENDING`状態でキューに入ったとき。 |
| `approval.decided` | 3 | 承認が`APPROVED`、`REJECTED`、または`EXECUTION_FAILED`に遷移したとき。 |

### `trigger.succeeded`

エージェントの実行がエラーなしで終了した後に発火します。ペイロードの`data`フィールドには以下が含まれます:

- `triggerId` - 実行の一意のID。
- `triggerType` - 実行を開始した[トリガー理由enum](#triggers-overview)。
- `status` - `SUCCESS`(文字列)。
- `tokensUsed` - この実行で消費されたトークン数。
- `wasDryRun` - エージェントが[ドライランモード](#dry-run-mode)だった場合はtrue。
- `actions` - `TenantAgentAction`レコードの配列([Webhook Payloads](#webhook-payloads)を参照)。
- `commentId`、`url`、`urlId` - トリガーにこれらがあった場合。

実行でアクションが行われなかった場合、`actions`配列は空になります - これは「エージェントが何もしないことを決定した」成功した実行であり、これを把握しておくと有用です。

### `trigger.failed`

実行がエラーになったときに発火します。ペイロードの構造は`trigger.succeeded`と同じですが、`status: 'ERROR'`となり、何が問題だったかを説明する追加の`errorMessage`フィールドが含まれます。発生しうるエラーには、LLM呼び出しの失敗、ツールディスパッチの失敗、実行中の予算枯渇などがあります。

`actions`には、エラーが発生する前に完了したツール呼び出しのエントリが含まれている場合があります。

### `approval.requested`

承認が`PENDING`状態でキューに入った瞬間に発火します。ペイロードには以下が含まれます:

- `approvalId`、`triggerId`。
- `toolName`、`actionType`。
- `status: 'PENDING'`。
- `args` - LLM呼び出しから**そのまま渡された**ツールの引数。構造はツールごとに異なり、安定した公開契約ではありません - 新しいツールが追加されるとスキーマは変更される可能性があります。
- `createdAt`。
- `justification`、`confidence` - エージェントが提供した場合。
- `contextSnapshot` - 承認が関連するコメント/ページのコンテキスト。

保留中の承認をチャットオペレーションチャンネルに転送するのに便利です:`approval.requested`を購読しているSlackボットは、アクションとその理由をモデレーションチャンネルに投稿し、一目で確認できるようにできます。

### `approval.decided`

承認が`PENDING`から移動したときに発火します。ペイロードには以下が含まれます:

- `approvalId`、`triggerId`。
- `toolName`、`actionType`。
- `status` - `APPROVED`、`REJECTED`、または`EXECUTION_FAILED`。
- `decidedBy` - 決定したモデレーターのユーザーID。
- `decidedAt` - 決定した日時。
- `executedAt` - APPROVEDの場合、プラットフォームが承認されたアクションを実行した日時。
- `executionResult` - APPROVEDの場合、実行結果を説明する文字列。
- `contextSnapshot` - コメント/ページのコンテキスト。

このイベントはすべての決定結果をカバーします:

- **承認 + 正常に実行** -> `status: APPROVED`、`executedAt`が設定され、`executionResult`は成功メッセージ。
- **承認 + 実行失敗** -> `status: EXECUTION_FAILED`、`executedAt`が設定され、`executionResult`は失敗の説明。
- **却下** -> `status: REJECTED`、`executedAt`はnull、`executionResult`はnull。

### ヘッダー

すべての配信には、イベントの正規の文字列名(`trigger.succeeded`など)を含む`X-FastComments-Agent-Event` HTTPヘッダーが含まれます。エンドポイントが複数のイベントタイプを処理する単一のURLである場合に便利です。

### 関連項目

- イベントごとの完全なペイロードスキーマについては[Webhook Payloads](#webhook-payloads)。
- HMACスキームについては[Webhook Signing](#webhook-signing)。
- 配信セマンティクスについては[Webhook Retries](#webhook-retries)。