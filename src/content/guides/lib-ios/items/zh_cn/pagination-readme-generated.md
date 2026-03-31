### 页面大小

```swift
// 评论：默认 30
sdk.pageSize = 50

// 信息流：默认 10
feedSDK.pageSize = 20
```

### 加载更多评论

UI 会自动显示分页控件。你也可以以编程方式触发分页：

```swift
// 加载下一页
try await sdk.loadMore()

// 加载所有剩余（若评论超过2000则为性能原因被禁用）
try await sdk.loadAll()

// 检查状态
sdk.hasMore            // 是否存在更多页面
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### 子评论分页

嵌套回复采用惰性加载。当用户展开一个线程时，会加载前 5 条子评论。如果存在更多，会出现“加载更多回复”控件。此功能由 UI 自动处理。