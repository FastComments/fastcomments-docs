### Спречавање цурења меморије

Да бисте спречили цурење меморије при коришћењу FastComments погледа у Activity-има или Fragment-има, увек позовите `cleanup()` када поглед више није потребан:

#### У Activity-има:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Очистите FastComments погледе да бисте спречили цурење меморије
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### У Fragment-има:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // Очистите FastComments погледе када се поглед фрагмента уништи
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

#### При мењању фрагмената:
```java
// Пре замене или уклањања фрагмента који садржи FastComments погледе
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Затим наставите са трансакцијом фрагмента
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Важно**: Увек позивајте `cleanup()` методе да бисте спречили цурење меморије, посебно када:
- Activity-ји се уништавају
- Погледи фрагмената се уништавају
- Прелазите између фрагмената
- Напуштате екране који садрже FastComments компоненте