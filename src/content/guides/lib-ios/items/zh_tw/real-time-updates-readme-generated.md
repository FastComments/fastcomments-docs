呼叫 `sdk.load()` 後，SDK 會自動訂閱已設定 `urlId` 的 WebSocket 事件。系統會處理以下事件：

- 新留言、編輯與刪除
- 投票（新增與移除）
- 釘選、鎖定、檢舉與封鎖狀態變更
- 使用者線上狀態（加入/離開）
- 討論串開啟/關閉
- 徽章頒發
- 伺服器設定更新

### 控制即時顯示

預設情況下，其他使用者的新留言會立即顯示：

```swift
sdk.showLiveRightAway = true   // Default: show instantly
```

將此設定為 `false` 可將新留言緩衝在「N 則新留言」按鈕後方，讓使用者自行選擇何時顯示：

```swift
sdk.showLiveRightAway = false
```

### 使用者線上狀態

當伺服器啟用線上狀態追蹤時，線上/離線指示器會自動顯示在使用者頭像上。用戶端無需額外設定。

---