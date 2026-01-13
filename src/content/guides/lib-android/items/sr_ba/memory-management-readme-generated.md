### Sprečavanje curenja memorije

Da biste spriječili curenje memorije pri korištenju FastComments views u Activities ili Fragments, uvijek pozovite `cleanup()` kada prikaz više nije potreban:

#### U Activities:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Očistite FastComments prikaze da spriječite curenje memorije
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### U Fragments:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // Očistite FastComments prikaze kada je prikaz fragmenta uništen
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Dodatno čišćenje kada je fragment uništen
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### Pri prebacivanju Fragments:
```java
// Prije zamjene ili uklanjanja fragmenta koji sadrži FastComments prikaze
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Zatim nastavite sa transakcijom fragmenta
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Važno**: Uvijek pozovite `cleanup()` metode da spriječite curenje memorije, posebno kada:
- Activities su uništene
- Fragment views su uništeni
- Prebacivanje između Fragments
- Napuštanje ekrana sa FastComments komponentama