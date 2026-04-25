---
對於大多數網站，新增評論最簡單的方法是將 `FastComments` 欄位附加到您的內容類型。前往 `Structure > Content types > [type] > Manage fields` 並新增該欄位。

Each entity that has the field gets:

- 一個 **狀態切換**，讓編輯可以對每個實體啟用或停用評論。
- 一個可選的 **自訂識別符**，讓您可以使用不綁定於 Drupal 實體路徑的穩定 ID。

The main `FastComments Widget` block knows about this field, and will skip entities that already have it attached. That way you can mix per-entity comments with the block without seeing the widget twice on the same page.
---