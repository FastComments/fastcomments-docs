Solid는 임의 객체에 대한 깊은 변경을 자동으로 추적하지 않으므로, 첫 렌더링 이후의 설정 변경은 명시적으로 푸시해야 합니다. 모든 위젯은 핸들을 반환하는 `apiRef`를 받습니다; `createEffect` 내에서 `handle.update(partial)`을 호출하여 반응성을 구동하세요:

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
- 스크립트가 로드되기 전: partial이 임시 저장되었다가 초기화 시 적용됩니다.
- 비동기 초기화 중(reviews-summary, user-activity-feed): partial이 큐에 저장되었다가 콜백이 완료될 때 적용됩니다.
- 초기화 후: 라이브 위젯의 `.update()` 메서드로 바로 전달됩니다.

### 명령형 핸들 API

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // 최신 라이브 인스턴스 (마운트 전에는 null)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // 인스턴스가 준비되면 한 번 호출됨
  update: (partial: Partial<Config>) => void; // 설정 병합 후 푸시
}
```

`.update()`로 커버되지 않는 명령형 작업(예: `openProfile`)에는 `getInstance()`를 사용하세요:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```