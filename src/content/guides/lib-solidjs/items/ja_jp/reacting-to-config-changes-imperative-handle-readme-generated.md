Solid は任意のオブジェクト上の深いミューテーションを自動的に追跡しないため、
最初のレンダー後の設定変更は明示的にプッシュする必要があります。各ウィジェットは
ハンドルを返す `apiRef` を受け取り、リアクティビティを駆動するために `createEffect` から
`handle.update(partial)` を呼び出します:

```tsx
import { createEffect, createSignal } from 'solid-js';
import { FastCommentsCommentWidget, type FastCommentsCommentWidgetHandle } from 'fastcomments-solidjs';

export default function Paginated() {
  const [page, setPage] = createSignal(0);
  let handle: FastCommentsCommentWidgetHandle | undefined;
  createEffect(() => handle?.update({ urlId: `product-${page()}` }));

  return (
    <>
      <button onClick={() => setPage(page() + 1)}>next</button>
      <FastCommentsCommentWidget
        apiRef={(h) => (handle = h)}
        tenantId="demo"
        urlId={`product-${page()}`}
      />
    </>
  );
}
```

`update()` はいつでも安全に呼び出せます:
- スクリプトがロードされる前: 部分的な設定は一時保存され、初期化時に適用されます。
- 非同期の初期化中（reviews-summary、user-activity-feed）: 部分的な設定はキューに入れられ、コールバックが解決されたときに適用されます。
- 初期化後: ライブウィジェットの `.update()` メソッドに直接転送されます。

### 命令的ハンドル API

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // latest live instance (or null before mount)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // fires once instance is ready
  update: (partial: Partial<Config>) => void; // merge-and-push config
}
```

`.update()` でカバーされない命令的な操作（例: `openProfile`）には `getInstance()` を使用します:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```