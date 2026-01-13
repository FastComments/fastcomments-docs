### 메모리 누수 방지

Activity 또는 Fragment에서 FastComments 뷰를 사용할 때 메모리 누수를 방지하려면, 더 이상 뷰가 필요하지 않을 때 항상 `cleanup()`을 호출하세요:

#### 액티비티에서:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // 메모리 누수를 방지하기 위해 FastComments 뷰를 정리합니다
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### 프래그먼트에서:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // 프래그먼트 뷰가 파괴될 때 FastComments 뷰를 정리합니다
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // 프래그먼트가 파괴될 때 추가 정리 수행
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### 프래그먼트를 전환할 때:
```java
// FastComments 뷰를 포함한 프래그먼트를 교체 또는 제거하기 전에
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Then proceed with fragment transaction
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**중요**: 메모리 누수를 방지하려면 항상 `cleanup()` 메서드를 호출하세요. 특히 다음과 같은 경우:
- 액티비티가 종료될 때
- 프래그먼트 뷰가 파괴될 때
- 프래그먼트 간 전환 시
- FastComments 컴포넌트를 포함한 화면에서 이동할 때