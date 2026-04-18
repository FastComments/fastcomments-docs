Solid 不会自动跟踪任意对象的深层变更，因此在首次渲染之后的配置更改必须显式推送。每个组件都接受返回一个句柄的 `apiRef`；在 `createEffect` 中调用 `handle.update(partial)` 以驱动响应性：

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

`update()` 在任何时候调用都是安全的：
- 在脚本加载之前：partial 会被暂存并在初始化时应用。
- 在异步初始化期间（reviews-summary、user-activity-feed）：partial 会被排队并在回调解析时应用。
- 初始化之后：它会直接转发到实时组件的 `.update()` 方法。

### 命令式句柄 API

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // 最新的活动实例（或挂载前为 null）
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // 实例准备好时触发一次
  update: (partial: Partial<Config>) => void; // 合并并推送配置
}
```

对于 `.update()` 未覆盖的命令式操作，例如 `openProfile`，请使用 `getInstance()`：

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```