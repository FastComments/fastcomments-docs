### Vermeidung von Speicherlecks

Um Speicherlecks zu vermeiden, rufen Sie beim Verwenden von FastComments-Views in Activities oder Fragments immer `cleanup()` auf, wenn die View nicht mehr benötigt wird:

#### In Activities:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // FastComments-Views bereinigen, um Speicherlecks zu verhindern
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### In Fragments:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // FastComments-Views bereinigen, wenn die Fragment-View zerstört wird
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Zusätzliche Bereinigung, wenn das Fragment zerstört wird
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### When Switching Fragments:
```java
// Vor dem Ersetzen oder Entfernen eines Fragments, das FastComments-Views enthält
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Dann mit der Fragment-Transaktion fortfahren
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Wichtig**: Rufen Sie immer die `cleanup()`-Methoden auf, um Speicherlecks zu verhindern, insbesondere wenn:
- Activities zerstört werden
- Fragment-Views zerstört werden
- Zwischen Fragments gewechselt wird
- Beim Verlassen von Bildschirmen, die FastComments-Komponenten enthalten