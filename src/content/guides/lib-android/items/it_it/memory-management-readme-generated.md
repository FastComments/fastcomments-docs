### Prevenzione delle perdite di memoria

Per prevenire perdite di memoria quando si usano le view di FastComments nelle Activity o nei Fragment, chiamare sempre `cleanup()` quando la view non è più necessaria:

#### Nelle Activity:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Pulire le view di FastComments per prevenire perdite di memoria
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### Nei Fragment:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // Pulire le view di FastComments quando la view del Fragment viene distrutta
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Pulizie aggiuntive quando il Fragment viene distrutto
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### Quando si sostituiscono i Fragment:
```java
// Prima di sostituire o rimuovere un Fragment che contiene le view di FastComments
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Quindi procedere con la transazione del Fragment
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Importante**: Chiamare sempre i metodi `cleanup()` per prevenire perdite di memoria, specialmente quando:
- Le Activity vengono distrutte
- Le view dei Fragment vengono distrutte
- Durante la sostituzione tra Fragment
- Quando si naviga lontano da schermate con componenti FastComments