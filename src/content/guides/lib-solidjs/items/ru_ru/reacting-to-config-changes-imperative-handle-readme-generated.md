Solid не отслеживает автоматически глубокие изменения произвольных объектов, поэтому
изменения конфигурации после первого рендера нужно передавать явно. Каждый виджет
принимает `apiRef`, который возвращает дескриптор; вызывайте `handle.update(partial)` из
`createEffect` для обеспечения реактивности:

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
- До загрузки скрипта: частичная конфигурация сохраняется и применяется при инициализации.
- Во время асинхронной инициализации (reviews-summary, user-activity-feed): частичная конфигурация ставится в очередь и применяется, когда колбэк завершится.
- После инициализации: вызов передаётся напрямую методу `.update()` живого виджета.

### Императивный API дескриптора

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // последний живой экземпляр (или null до монтирования)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // срабатывает один раз, когда экземпляр готов
  update: (partial: Partial<Config>) => void; // объединение и отправка конфигурации
}
```

Используйте `getInstance()` для императивных действий, не покрытых `.update()`, например `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```