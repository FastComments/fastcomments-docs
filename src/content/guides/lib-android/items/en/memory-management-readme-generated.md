### Preventing Memory Leaks

To prevent memory leaks when using FastComments views in Activities or Fragments, always call `cleanup()` when the view is no longer needed:

#### In Activities:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Clean up FastComments views to prevent memory leaks
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
    // Clean up FastComments views when fragment view is destroyed
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Additional cleanup when fragment is destroyed
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### When Switching Fragments:
```java
// Before replacing or removing a fragment containing FastComments views
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Then proceed with fragment transaction
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Important**: Always call `cleanup()` methods to prevent memory leaks, especially when:
- Activities are destroyed
- Fragment views are destroyed
- Switching between fragments
- Navigating away from screens with FastComments components