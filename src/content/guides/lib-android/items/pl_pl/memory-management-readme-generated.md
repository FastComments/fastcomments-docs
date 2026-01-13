### Zapobieganie wyciekom pamięci

Aby zapobiec wyciekom pamięci podczas korzystania z widoków FastComments w Activities lub Fragments, zawsze wywołuj `cleanup()` gdy widok nie jest już potrzebny:

#### W Activities:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Oczyść widoki FastComments, aby zapobiec wyciekom pamięci
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### W Fragments:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // Oczyść widoki FastComments, gdy widok fragmentu zostanie zniszczony
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Dodatkowe czyszczenie przy niszczeniu fragmentu
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### Podczas przełączania Fragments:
```java
// Przed zastąpieniem lub usunięciem fragmentu zawierającego widoki FastComments
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Następnie kontynuuj transakcję fragmentu
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Ważne**: Zawsze wywołuj metody `cleanup()` aby zapobiec wyciekom pamięci, szczególnie gdy:
- Activities są niszczone
- Fragment views są niszczone
- Przełączanie między fragments
- Nawigowanie z ekranów zawierających komponenty FastComments