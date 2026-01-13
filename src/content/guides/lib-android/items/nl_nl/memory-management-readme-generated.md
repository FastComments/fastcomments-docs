### Geheugenlekken voorkomen

Om geheugenlekken te voorkomen bij het gebruik van FastComments-weergaven in activiteiten of fragmenten, moet je altijd `cleanup()` aanroepen wanneer de view niet langer nodig is:

#### In activiteiten:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Ruim FastComments-weergaven op om geheugenlekken te voorkomen
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### In fragmenten:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // Ruim FastComments-weergaven op wanneer de view van het fragment wordt vernietigd
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Extra opruiming wanneer het fragment wordt vernietigd
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### Bij het wisselen van fragmenten:
```java
// Voordat je een fragment vervangt of verwijdert dat FastComments-weergaven bevat
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Then proceed with fragment transaction
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Belangrijk**: Roep altijd de `cleanup()`-methoden aan om geheugenlekken te voorkomen, vooral wanneer:
- Activiteiten worden vernietigd
- Fragment-views worden vernietigd
- Er wordt gewisseld tussen fragmenten
- Je navigeert weg van schermen met FastComments-componenten