### Forebyggelse af hukommelseslækager

For at forhindre hukommelseslækager når du bruger FastComments-visninger i Aktiviteter eller Fragmenter, kald altid `cleanup()` når visningen ikke længere er nødvendig:

#### I Aktiviteter:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Ryd op i FastComments-visninger for at forhindre hukommelseslækager
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### I Fragmenter:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // Ryd op i FastComments-visninger når fragmentets visning ødelægges
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Yderligere oprydning når fragmentet ødelægges
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### Ved skift mellem fragmenter:
```java
// Før du erstatter eller fjerner et fragment, der indeholder FastComments-visninger
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Fortsæt derefter med fragmenttransaktionen
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Vigtigt**: Kald altid `cleanup()`-metoder for at forhindre hukommelseslækager, især når:
- Aktiviteter bliver ødelagt
- Fragmentvisninger bliver ødelagt
- Skift mellem fragmenter
- Navigering væk fra skærme med FastComments-komponenter