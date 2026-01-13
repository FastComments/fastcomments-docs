### Запобігання витокам пам'яті

Щоб запобігти витокам пам'яті під час використання представлень FastComments у Activities або Fragments, завжди викликайте `cleanup()`, коли представлення більше не потрібне:

#### У Activities:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Очистіть представлення FastComments, щоб запобігти витокам пам'яті
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### У Fragments:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // Очистіть представлення FastComments, коли view фрагмента знищується
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Додаткове очищення при знищенні фрагмента
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### Під час переключення фрагментів:
```java
// Перед заміною або видаленням фрагмента, що містить представлення FastComments
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Потім продовжте транзакцію фрагмента
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Важливо**: Завжди викликайте методи `cleanup()` для запобігання витокам пам'яті, особливо коли:
- Activity знищуються
- Подання фрагментів знищуються
- Перемикання між фрагментами
- Перехід з екранів з компонентами FastComments