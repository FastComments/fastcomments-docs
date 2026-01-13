### Bellek Sızıntılarını Önleme

Activity'lerde veya Fragment'larda FastComments görünümlerini kullanırken bellek sızıntılarını önlemek için, görünüm artık ihtiyaç kalmadığında her zaman `cleanup()` çağırın:

#### Activity'lerde:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Bellek sızıntılarını önlemek için FastComments görünümlerini temizleyin
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### Fragment'larda:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // Fragment görünümü yok edildiğinde FastComments görünümlerini temizleyin
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Fragment yok edildiğinde ek temizleme
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### Fragment'ler Arasında Geçiş Yaparken:
```java
// FastComments görünümleri içeren bir fragment'i değiştirmeden veya kaldırmadan önce
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Ardından fragment işlemiyle devam edin
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Önemli**: Bellek sızıntılarını önlemek için her zaman `cleanup()` metodlarını çağırın, özellikle:
- Activity'ler yok edildiğinde
- Fragment görünümleri yok edildiğinde
- Fragment'ler arasında geçiş yapıldığında
- FastComments bileşenleri içeren ekranlardan ayrıldığınızda