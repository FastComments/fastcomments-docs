### 格式範例

定時留言CSV看起來像這樣:

|ID |Parent ID|URL ID       |URL|Name         |Avatar|Comment                              | Hours | Minutes | Seconds |
|---|---------|-------------|---|-------------|------|-------------------------------------|-------|---------|---------|
|1  |         |scheduledtest|   |Test Person 1|      |Hello!                               | 0     | 0       | 3       |
|2  |1        |scheduledtest|   |Test Person 2|      |How are you?                         | 0     | 0       | 10      |
|3  |2        |scheduledtest|   |Test Person 3|      |Leave them alone                     | 0     | 1       | 30      |
|4  |         |scheduledtest|   |Test Person 4|      |Does anyone have a cute cat to share?| 1     | 10      | 0       |


### 格式詳情

定時留言CSV檔案支援以下欄位:

- ID
- Parent ID
- URL ID
- URL
- Name
- Avatar
- Comment
- Hours
- Minutes
- Seconds

以下欄位是**必需的**:

- ID
- URL ID
- Name
- Comment
- Hours
- Minutes
- Seconds

如果您想支援自動化巢狀回覆,您將需要Parent ID欄位。

### 格式如何運作

匯入格式的運作方式如下:

1. 您在CSV中為要發布的每條留言定義一行。
2. 留言在指定的延遲(小時 + 分鐘 + 秒)後發布。
   1. 對於手動計劃的匯入,延遲是相對於您在匯入完成後在UI中點擊"play"的時間 - **不是匯入開始的時間**。
   2. 對於自動計劃的匯入,延遲是從頁面載入時開始。
3. 您必須定義一個ID。您可以簡單地使用遞增的識別碼,如1、2、3、4、5。
4. 如果您想使用巢狀回覆,只需將`Parent ID`欄位值設定為另一條留言的`ID`。
5. `Comment`欄位支援FastComments在其留言小工具中支援的相同語法,用於文字樣式和新增圖片。
6. `Avatar`欄位(如果使用)必須是可公開存取的圖片。它將被複製到我們的CDN並從那裡提供服務。
7. 您可以在播放後刪除所有留言,或者如果播放停止。
8. 刪除是即時發生的,因此您可以反覆重複使用相同的計劃匯入。

### 範例

[範例CSV檔案在這裡](/csv/fastcomments-scheduled-comments-example.csv)。
