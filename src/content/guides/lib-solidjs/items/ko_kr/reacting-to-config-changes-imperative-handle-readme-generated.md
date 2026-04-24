Solid은 임의 객체의 깊은 변경을 자동으로 추적하지 않으므로
첫 렌더 이후의 config 변경은 명시적으로 푸시해야 합니다. 모든 위젯은
핸들을 반환하는 `apiRef`를 허용합니다; 반응성을 구동하려면 `createEffect`에서
`handle.update(partial)`를 호출하세요:

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

`update()`는 언제든지 호출해도 안전합니다:
- Before the script has loaded: the partial is stashed and applied at init.
- During an async init (reviews-summary, user-activity-feed): the partial is queued and applied when the callback resolves.
- After init: it forwards straight to the live widget's `.update()` method.

### 명령형 핸들 API

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // 최신 라이브 인스턴스(또는 마운트 전에는 null)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // 인스턴스가 준비되면 한 번 호출됨
  update: (partial: Partial<Config>) => void; // 구성 병합 및 푸시
}
```

`.update()`로 다루어지지 않는 명령형 동작에는 `getInstance()`를 사용하세요. 예: `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```