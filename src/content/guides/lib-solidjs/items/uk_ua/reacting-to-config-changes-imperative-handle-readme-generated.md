Solid не відстежує автоматично глибокі мутації в довільних об'єктах, тому
зміни конфігурації після першого рендеру потрібно передавати явно. Кожен віджет
приймає `apiRef`, який повертає дескриптор; викликайте `handle.update(partial)` з
`createEffect`, щоб забезпечити реактивність:

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

`update()` безпечно викликати в будь-який момент:
- Before the script has loaded: the partial is stashed and applied at init.
- During an async init (reviews-summary, user-activity-feed): the partial is queued and applied when the callback resolves.
- After init: it forwards straight to the live widget's `.update()` method.

### Імперативний API дескриптора

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // останній активний екземпляр (або null до монтування)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // викликається, коли екземпляр готовий
  update: (partial: Partial<Config>) => void; // зливає і передає конфігурацію
}
```

Використовуйте `getInstance()` для імперативних дій, які не покриває `.update()`, наприклад `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```