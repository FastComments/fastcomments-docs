### 頁面大小

```swift
// 評論：預設 30
sdk.pageSize = 50

// Feed：預設 10
feedSDK.pageSize = 20
```

### 載入更多評論

使用者介面會自動顯示分頁控制項。您也可以以程式觸發分頁：

```swift
// 載入下一頁
try await sdk.loadMore()

// 載入所有剩餘（若超過 2000 則為效能考量而停用）
try await sdk.loadAll()

// 檢查狀態
sdk.hasMore            // 是否還有更多頁面
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### 子評論分頁

巢狀回覆採延遲載入。當使用者展開討論串時，會載入前 5 個子回覆。如果還有更多回覆，會顯示「載入更多回覆」控制項。此行為由使用者介面自動處理。

---
---