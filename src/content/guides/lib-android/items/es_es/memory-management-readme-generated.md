### Prevención de fugas de memoria

Para evitar fugas de memoria al usar vistas de FastComments en Actividades o Fragmentos, siempre llama a `cleanup()` cuando la vista ya no sea necesaria:

#### En Actividades:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Limpiar vistas de FastComments para evitar fugas de memoria
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### En Fragmentos:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // Limpiar vistas de FastComments cuando la vista del fragmento se destruye
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Limpieza adicional cuando el fragmento se destruye
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### Al cambiar de Fragmentos:
```java
// Antes de reemplazar o quitar un fragmento que contenga vistas de FastComments
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Luego continúa con la transacción de fragmentos
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Importante**: Siempre llama a los métodos `cleanup()` para evitar fugas de memoria, especialmente cuando:
- Se destruyen las Actividades
- Se destruyen las vistas de los Fragmentos
- Al cambiar entre Fragmentos
- Al navegar fuera de pantallas con componentes de FastComments