### Speicherlecks verhindern

Um Speicherlecks bei der Verwendung von FastComments-Views in Activities oder Fragments zu vermeiden, rufen Sie immer `cleanup()` auf, wenn die View nicht mehr benötigt wird:

#### In Activities:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // FastComments-Views aufräumen, um Speicherlecks zu vermeiden
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
    // FastComments-Views aufräumen, wenn die Fragment-View zerstört wird
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Zusätzliches Aufräumen, wenn das Fragment zerstört wird
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### Beim Wechseln von Fragments:
```java
// Bevor ein Fragment mit FastComments-Views ersetzt oder entfernt wird
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Dann mit der Fragment-Transaktion fortfahren
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Wichtig**: Rufen Sie immer die `cleanup()`-Methoden auf, um Speicherlecks zu vermeiden, insbesondere wenn:
- Activities zerstört werden
- Fragment-Views zerstört werden
- Zwischen Fragments gewechselt wird
- Von Bildschirmen mit FastComments-Komponenten weg navigiert wird