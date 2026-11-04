Solid не відстежує автоматично глибокі мутації довільних об'єктів, тому зміни конфігурації після першого рендеру потрібно передавати явно. Кожен віджет приймає `apiRef`, який повертає дескриптор; викликайте `handle.update(partial)` з `createEffect`, щоб керувати реактивністю:

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

`update()` безпечно викликати будь-коли:
- До завантаження скрипту: часткові зміни зберігаються і застосовуються під час ініціалізації.
- Під час асинхронної ініціалізації (reviews-summary, user-activity-feed): часткові зміни ставляться в чергу і застосовуються, коли колбек завершиться.
- Після ініціалізації: вони передаються безпосередньо до методу `.update()` активного віджета.

### Імперативний API дескриптора

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // останній активний екземпляр (або null до монтування)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // спрацьовує один раз, коли екземпляр готовий
  update: (partial: Partial<Config>) => void; // об'єднати та надіслати конфігурацію
}
```

Використовуйте `getInstance()` для імперативних дій, які не покриваються `.update()`, наприклад `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```