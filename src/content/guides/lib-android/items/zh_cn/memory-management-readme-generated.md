### 防止内存泄漏

在 Activity 或 Fragment 中使用 FastComments 视图时，为防止内存泄漏，当视图不再需要时务必调用 `cleanup()`：

#### 在 Activity 中：
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // 清理 FastComments 视图以防止内存泄漏
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
    // 在 fragment 视图被销毁时清理 FastComments 视图
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // 在 fragment 被销毁时进行额外清理
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### 切换 Fragment 时：
```java
// 在替换或移除包含 FastComments 视图的 fragment 之前
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// 然后继续进行 fragment 事务
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**重要**：务必调用 `cleanup()` 方法以防止内存泄漏，尤其是在以下情况下：
- Activity 被销毁时
- Fragment 视图被销毁时
- 在 Fragment 之间切换时
- 从包含 FastComments 组件的屏幕导航离开时