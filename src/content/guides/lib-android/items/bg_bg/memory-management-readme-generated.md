### Предотвратяване на изтичания на памет

За да предотвратите изтичания на памет при използване на FastComments изгледи в Activities или Fragments, винаги извиквайте `cleanup()` когато изгледът вече не е необходим:

#### В Activities:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Почистете FastComments изгледите, за да предотвратите изтичания на паметта
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
    // Почистете FastComments изгледите когато изгледът на фрагмента бъде унищожен
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Допълнително почистване когато фрагментът бъде унищожен
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### При смяна на Fragments:
```java
// Преди да замените или премахнете фрагмент, съдържащ FastComments изгледи
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// След това продължете с транзакцията на фрагмента
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Важно**: Винаги извиквайте методите `cleanup()`, за да предотвратите изтичания на паметта, особено когато:
- Activities се унищожават
- Изгледите на Fragments се унищожават
- Смяна между Fragments
- Напускане на екрани с FastComments компоненти