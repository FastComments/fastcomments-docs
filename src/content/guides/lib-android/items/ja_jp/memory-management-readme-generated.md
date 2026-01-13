### メモリリークの防止

Activities または Fragments で FastComments ビューを使用する際にメモリリークを防ぐため、ビューが不要になったら常に `cleanup()` を呼び出してください:

#### アクティビティ内:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // メモリリークを防ぐため、FastComments ビューをクリーンアップします
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### フラグメント内:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // フラグメントのビューが破棄されたときに FastComments ビューをクリーンアップします
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // フラグメントが破棄されたときの追加のクリーンアップ
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### フラグメントを切り替えるとき:
```java
// FastComments ビューを含むフラグメントを置換または削除する前に
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// その後、フラグメントトランザクションを実行します
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**重要**: メモリリークを防ぐために常に `cleanup()` メソッドを呼び出してください。特に次の場合:
- アクティビティが破棄されるとき
- フラグメントのビューが破棄されるとき
- フラグメントを切り替えるとき
- FastComments コンポーネントを含む画面から離れるとき