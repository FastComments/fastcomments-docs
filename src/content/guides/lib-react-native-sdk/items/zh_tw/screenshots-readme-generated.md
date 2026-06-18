---
具有頭像、巢狀回覆、投票和內建富文本撰寫器的即時線程評論，另外包含深色主題和即時聊天預設（此處透過 `react-native-web` 呈現）：

<table>
  <tr>
    <td align="center"><b>即時評論</b><br/><img src="./demo-screenshots/light.png" width="260" alt="即時評論，淺色主題"/></td>
    <td align="center"><b>深色主題</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="即時評論，深色主題"/></td>
    <td align="center"><b>即時聊天</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="即時聊天預設"/></td>
  </tr>
</table>

### 富文本編輯器

本函式庫使用 [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) 進行富文本編輯，提供功能強大的所見即所得 (WYSIWYG) 編輯體驗。相同的編輯器驅動 iOS、Android 與網頁（透過 `react-native-web`），因此撰寫器在每個平台上以單一實作保持一致行為。

`react-native-enriched` 在原生端需要 React Native 新架構 (Fabric)（自 RN 0.76 起為預設，在 RN 0.72-0.75 需手動啟用），以及能解析套件 `exports` 條件的打包器。此 SDK 以 RN 0.81 / React 19 為開發與測試目標。同一編輯器也透過 `react-native-web` 在網頁上運行；enriched 編輯器的網頁建置在上游仍被標記為實驗性。

### 元件

此 SDK 提供三個元件，與 FastComments Android SDK 對應：

- `FastCommentsLiveCommenting` - 具有投票、回覆、分頁、提及、通知與即時更新的線程式評論。
- `FastCommentsLiveChat` - 使用相同引擎的聊天預設：訊息按時間順序排列，最新訊息在底部，撰寫器位於列表下方，有即時的標頭條（連線點 + 使用者數），透過向上捲動載入無限歷史，會自動捲動到新訊息，沒有投票或回覆線程。每個預設都可透過 `config` 覆寫。
- `FastCommentsFeed` - 一個社交動態，包含發文撰寫器、多媒體、反應、關注，以及即時新貼文橫幅。

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### 主題

預設外觀是從一組語義化設計代幣 (`FastCommentsTheme`) 產生：顏色、間距、圓角、字型大小、字重與頭像尺寸。在任一元件的 `theme` prop 傳入部分代幣覆寫（型別為 `FastCommentsThemeOverrides`），整個樣式樹將一致地重新套用樣式：

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

啟用深色模式只需一組代幣：

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` prop 仍接受原始的 `IFastCommentsStyles` 樹以供精細控制。當同時提供 `theme` 與 `styles` 時，明確定義的 styles 會優先於主題化的樣式樹；當僅提供 `styles` 時，它將完全取代預設（這是原有行為，因此現有的整合與樣式不會受影響）。`setupDarkModeSkin` 已不建議使用，改為使用 `theme` prop。

### 配置選項

本函式庫旨在支援 [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) 中定義的所有配置選項，與網頁實作一致。

除此之外，React Native 透過 `FastCommentsRNConfig` 增加了一些 SDK 專用選項：

- `hideTopBar` - 隱藏顯示於撰寫器上方的已登入使用者/通知鈴鐺列。
- `usePressToEdit` - 長按評論以開啟其選單。
- `disableDownVoting` - 隱藏倒票（down-vote）按鈕。
- `renderCommentInline` - 將評論者資訊渲染在與評論內容相同的 HTML 區塊內。
- `renderLikesToRight` - 將投票/喜歡區域移到評論的右側，而非下方。
- `renderDateBelowComment` - 在評論下方顯示日期。
- `showLiveStatus` - 在評論上方顯示類似聊天樣式的「Live」＋使用者數量標頭條。
- `useInlineSubmitButton` - 將送出按鈕以圖示方式呈現在撰寫器內。
- `countAboveToggle` - 與 `useShowCommentsToggle` 一起使用時，指定在「顯示評論」切換按鈕上方呈現多少則評論。
- `preserveFeedScrollPosition` - `FastCommentsFeed` 在卸載/重新掛載間記住其捲動位移（預設 true）。

### FastComments 概念

開始使用時主要要了解的概念是 `tenantId` 與 `urlId`。`tenantId` 是你的 FastComments.com 帳戶識別碼。`urlId` 是評論串所綁定的對象，這可以是頁面 URL，或產品 id、文章 id 等。

### 使用者通知

FastComments 支援 [多種情境](https://docs.fastcomments.com/guide-notifications.html) 的通知。通知可設定，使用者可在全域或在單一通知/評論層級選擇取消訂閱，並支援頁面層級的訂閱，讓使用者能訂閱特定頁面或文章的討論串。

例如，可以使用 Secure SSO 來驗證使用者，接著定期輪詢未讀通知並推送給使用者。

請參閱 [範例 AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) 了解如何取得與轉換使用者未讀通知。

### Gif 瀏覽器

預設不啟用任何圖片或 gif 選擇功能。請參閱 [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) 了解如何支援圖片與 gif 上傳。此函式庫提供一個會將搜尋與圖片匿名化的 Gif 瀏覽器，你只需使用它即可。

### 效能

若發現任何效能問題，請建立一個包含可重現範例與所使用裝置的 issue。效能在所有 FastComments 函式庫中都是首要考量。
---