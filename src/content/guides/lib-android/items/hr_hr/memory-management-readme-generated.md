### Sprječavanje curenja memorije

Da biste spriječili curenje memorije pri korištenju FastComments prikaza u aktivnostima ili fragmentima, uvijek pozovite `cleanup()` kada prikaz više nije potreban:

#### U aktivnostima:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Očistite FastComments prikaze kako biste spriječili curenje memorije
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
    // Očistite FastComments prikaze kada se prikaz fragmenta uništi
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

#### Prilikom prebacivanja fragmenta:
```java
// Prije zamjene ili uklanjanja fragmenta koji sadrži FastComments prikaze
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Zatim nastavite s transakcijom fragmenta
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Važno**: Uvijek pozovite `cleanup()` metode kako biste spriječili curenje memorije, osobito kada:
- aktivnosti se unište
- prikazi fragmenata se unište
- pri prebacivanju između fragmenata
- pri napuštanju zaslona koji sadrže FastComments komponente