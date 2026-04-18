Solid не отслеживает автоматически глубокие мутации произвольных объектов, поэтому изменения config после первого рендера должны быть отправлены явно. Каждый виджет принимает `apiRef`, который возвращает handle; вызовите `handle.update(partial)` внутри `createEffect`, чтобы обеспечить реактивность:

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

`update()` безопасно вызывать в любой момент:
- До загрузки скрипта: partial сохраняется и применяется при init.
- Во время асинхронной инициализации (reviews-summary, user-activity-feed): partial ставится в очередь и применяется, когда callback разрешится.
- После init: он напрямую перенаправляется в метод `.update()` живого виджета.

### Императивный handle API

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // последний живой экземпляр (или null до монтирования)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // срабатывает один раз, когда экземпляр готов
  update: (partial: Partial<Config>) => void; // объединяет и отправляет конфигурацию
}
```

Используйте `getInstance()` для императивных действий, которые не покрываются `.update()`, например `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```