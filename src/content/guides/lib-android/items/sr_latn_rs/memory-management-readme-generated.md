### Sprečavanje curenja memorije

Da biste sprečili curenje memorije prilikom korišćenja FastComments view-ova u aktivnostima ili fragmentima, uvek pozovite `cleanup()` kada view više nije potreban:

#### U aktivnostima:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Očistite FastComments view-e da biste sprečili curenje memorije
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### U fragmentima:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // Očistite FastComments view-e kada se view fragmenta uništi
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Dodatno čišćenje kada se fragment uništi
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### Pri menjanju fragmenata:
```java
// Pre nego što zamenite ili uklonite fragment koji sadrži FastComments view-e
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Zatim nastavite sa transakcijom fragmenta
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Važno**: Uvek pozivajte metode `cleanup()` da biste sprečili curenje memorije, posebno kada:
- aktivnosti budu uništene
- view-i fragmenta budu uništeni
- menjate fragmente
- napuštate ekrane sa FastComments komponentama