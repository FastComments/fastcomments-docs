### Предотвращение утечек памяти

Чтобы предотвратить утечки памяти при использовании представлений FastComments в Activities или Fragments, всегда вызывайте `cleanup()`, когда представление больше не требуется:

#### В Activities:
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

#### В Fragments:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // Очистите представления FastComments при уничтожении view фрагмента
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

#### При переключении Fragments:
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

**Важно**: Всегда вызывайте методы `cleanup()`, чтобы предотвратить утечки памяти, особенно в следующих случаях:
- Activities уничтожаются
- Представления Fragment уничтожаются
- Переключение между fragments
- Переход с экранов, содержащих компоненты FastComments