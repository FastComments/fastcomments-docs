### 防止記憶體洩漏

為了避免在 Activity 或 Fragment 中使用 FastComments 檢視時發生記憶體洩漏，請在檢視不再需要時務必呼叫 `cleanup()`:

#### 在 Activity 中：
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

#### 在 Fragment 中：
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

#### 切換 Fragment 時：
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

**重要**：請務必呼叫 `cleanup()` 方法以防止記憶體洩漏，特別是在以下情況：
- Activity 被銷毀時
- Fragment 檢視被銷毀時
- 在 Fragment 之間切換時
- 離開包含 FastComments 元件的畫面時