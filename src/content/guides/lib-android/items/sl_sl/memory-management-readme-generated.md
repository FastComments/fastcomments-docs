### Preprečevanje puščanja pomnilnika

Da preprečite puščanje pomnilnika pri uporabi pogledov FastComments v aktivnostih ali fragmentih, vedno pokličite `cleanup()`, ko pogled ni več potreben:

#### V aktivnostih:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Počistite FastComments poglede, da preprečite puščanje pomnilnika
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### V fragmentih:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // Počistite FastComments poglede, ko je pogled fragmenta uničen
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Dodatno čiščenje, ko je fragment uničen
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### Pri preklapljanju fragmentov:
```java
// Pred zamenjavo ali odstranitvijo fragmenta, ki vsebuje poglede FastComments
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Nato nadaljujte s transakcijo fragmenta
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Pomembno**: Vedno pokličite metode `cleanup()`, da preprečite puščanje pomnilnika, še posebej ko:
- ko so aktivnosti uničene
- ko so pogledi fragmentov uničeni
- pri preklapljanju fragmentov
- ko zapuščate zaslone s komponentami FastComments