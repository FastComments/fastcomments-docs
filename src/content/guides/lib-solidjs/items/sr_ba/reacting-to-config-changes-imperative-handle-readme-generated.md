Solid аутоматски не прати дубоке измене на произвољним објектима, па измене конфигурације након првог рендера морају бити експлицитно послате. Сваки видгет прихвата `apiRef` који враћа хендл; позовите `handle.update(partial)` из `createEffect` да бисте покренули реактивност:

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

`update()` је безбједан за позив у било које вријеме:
- Пре него што се скрипта учита: partial се привремено складишти и примјењује при иницијализацији.
- Током асинхроне иницијализације (reviews-summary, user-activity-feed): partial се ставља у ред и примјењује када callback заврши.
- Након иницијализације: он просљеђује директно на живи видгетову `.update()` методу.

### Императивни handle API

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // најновија активна инстанца (или null прије монтирања)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // позива се када је инстанца спремна
  update: (partial: Partial<Config>) => void; // спајање и слање конфигурације
}
```

Use `getInstance()` for imperative actions that aren't covered by `.update()`, e.g. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```