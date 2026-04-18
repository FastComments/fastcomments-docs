Solid 不會自動追蹤任意物件的深層變更，因此在第一次渲染後的 config 變更必須被明確推送。每個 widget 都接受一個會回傳 handle 的 `apiRef`；在 `createEffect` 中呼叫 `handle.update(partial)` 以驅動反應性：

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

`update()` 在任何時候呼叫都是安全的：
- 在 script 載入之前：partial 會被暫存並在初始化時套用。
- 在非同步初始化期間 (reviews-summary, user-activity-feed)：partial 會排入佇列，並在 callback 解決時套用。
- 在初始化之後：它會直接轉發到 live widget 的 `.update()` 方法。

### 命令式 handle API

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // 最新的運行中實例（或在掛載前為 null）
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // 在實例就緒時觸發一次
  update: (partial: Partial<Config>) => void; // 合併並推送 config
}
```

對於 `.update()` 無法涵蓋的命令式操作，請使用 `getInstance()`，例如 `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```