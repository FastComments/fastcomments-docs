### Prévention des fuites de mémoire

Pour éviter les fuites de mémoire lors de l'utilisation des vues FastComments dans Activities ou Fragments, appelez toujours `cleanup()` lorsque la vue n'est plus nécessaire :

#### Dans les Activities :
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Nettoyer les vues FastComments pour éviter les fuites de mémoire
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### Dans les Fragments :
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // Nettoyer les vues FastComments lorsque la vue du fragment est détruite
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Nettoyage supplémentaire lorsque le fragment est détruit
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### Lors du changement de Fragments :
```java
// Avant de remplacer ou de supprimer un fragment contenant des vues FastComments
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Procéder ensuite à la transaction de fragment
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Important** : Appelez toujours les méthodes `cleanup()` pour éviter les fuites de mémoire, en particulier lorsque :
- Activities sont détruites
- Les vues des fragments sont détruites
- Changement entre fragments
- Navigation hors des écrans contenant des composants FastComments