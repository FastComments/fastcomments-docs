### Предотвращение утечек памяти

Чтобы предотвратить утечки памяти при использовании представлений FastComments в Activity или Fragment, всегда вызывайте `cleanup()`, когда представление больше не требуется:

#### В Activity:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Очистите представления FastComments, чтобы предотвратить утечки памяти
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### В Fragment:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // Очистите представления FastComments, когда view фрагмента уничтожается
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Дополнительная очистка при уничтожении фрагмента
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### При переключении фрагментов:
```java
// Перед заменой или удалением фрагмента, содержащего представления FastComments
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Затем выполните транзакцию фрагмента
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Важно**: Всегда вызывайте методы `cleanup()`, чтобы предотвратить утечки памяти, особенно когда:
- Activity уничтожаются
- Представления фрагмента уничтожаются
- При переключении между фрагментами
- При переходе с экранов, содержащих компоненты FastComments