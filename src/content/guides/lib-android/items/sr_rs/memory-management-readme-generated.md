### Спречавање цурења меморије

Да бисте спречили цурење меморије при коришћењу FastComments приказа у Activities или Fragments, увек позовите `cleanup()` када приказ више није потребан:

#### У Activities:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Очистите FastComments приказе да бисте спречили цурење меморије
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
    // Очистите FastComments приказе када се view фрагмента уништи
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Додатно чишћење када се фрагмент уништи
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### При пребацивању фрагмената:
```java
// Пре него што замените или уклоните фрагмент који садржи FastComments приказе
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id/container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Then proceed with fragment transaction
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Важно**: Увек позивајте методе `cleanup()` да бисте спречили цурење меморије, посебно када:
- Activities се уништавају
- Fragment views се уништавају
- Пребацивање између fragments
- Напуштање екрана са FastComments компонентама