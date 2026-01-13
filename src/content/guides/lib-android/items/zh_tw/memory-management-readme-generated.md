### 防止記憶體洩漏

為了在 Activity 或 Fragment 中使用 FastComments 視圖時防止記憶體洩漏，當不再需要該視圖時，請務必呼叫 `cleanup()`：

#### In Activities:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // 清理 FastComments 視圖以防止記憶體洩漏
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
    // 當 Fragment 視圖被銷毀時，清理 FastComments 視圖
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Fragment 被銷毀時的額外清理
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### When Switching Fragments:
```java
// 在替換或移除包含 FastComments 視圖的 Fragment 之前
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// 然後繼續執行 Fragment 交易
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**重要**：請務必在以下情況呼叫 `cleanup()` 方法以防止記憶體洩漏，特別是在：
- Activities 被銷毀時
- Fragment 視圖被銷毀時
- 切換 Fragment 時
- 從包含 FastComments 元件的畫面導覽離開時