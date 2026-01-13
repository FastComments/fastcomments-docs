### Prevenção de Vazamentos de Memória

Para prevenir vazamentos de memória ao usar as views do FastComments em Activities ou Fragments, sempre chame `cleanup()` quando a view não for mais necessária:

#### Em Activities:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Limpar as views do FastComments para prevenir vazamentos de memória
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### Em Fragments:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // Limpar as views do FastComments quando a view do fragment for destruída
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Limpeza adicional quando o fragment for destruído
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### Ao Alternar Fragments:
```java
// Antes de substituir ou remover um fragment que contenha views do FastComments
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Then proceed with fragment transaction
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Importante**: Sempre chame os métodos `cleanup()` para evitar vazamentos de memória, especialmente quando:
- Activities são destruídas
- As views de Fragment são destruídas
- Ao alternar entre fragments
- Ao navegar para fora de telas com componentes do FastComments