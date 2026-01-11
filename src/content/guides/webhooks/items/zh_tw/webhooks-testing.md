在 Webhooks admin 中有每個事件類型的 `Send Test Payload` 按鈕（Create、Update、Delete）。Create 和 Update 事件會送出一個假的 WebhookComment 物件，而測試 Delete 時會送出一個只包含 ID 的假請求主體。

測試會進行兩次呼叫，以驗證在 "happy"（correct API Key）和 "sad"（invalid API key）情境下的回應狀態碼。

當測試送出 invalid API key 時，你應回傳狀態碼 401，才能讓測試完全通過。如果你沒有正確檢查 token 的值，將會看到錯誤。

這是為了確保你正確驗證該請求。