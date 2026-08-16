Live threaded commenting with avatars, nested replies, votes, and the built‑in rich‑text composer, plus a dark theme and a live‑chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>即時評論</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="即時評論，淺色主題"/></td>
    <td align="center"><b>深色主題</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="即時評論，深色主題"/></td>
    <td align="center"><b>即時聊天</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="即時聊天預設"/></td>
  </tr>
</table>

### 富文字編輯器

此函式庫使用 [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) 進行富文字編輯，提供強大的所見即所得編輯體驗。相同的編輯器支援 iOS、Android 與網頁（透過 `react-native-web`），因此編輯器在所有平台上皆以單一實作保持一致的行為。

`react-native-enriched` 需要在原生端使用 React Native 新架構 (Fabric)（自 RN 0.76 起為預設，於 RN 0.72‑0.75 需自行啟用），以及能解析套件 `exports` 條件的打包工具。此 SDK 以 RN 0.81 / React 19 為開發與測試基礎。相同的編輯器亦可透過 `react-native-web` 在網頁上執行；enriched 編輯器的網頁版仍被標示為上游實驗性功能。

### 小工具

此 SDK 提供三個小工具，與 FastComments Android SDK 相同：

- `FastCommentsLiveCommenting` - 具備投票、回覆、分頁、提及、通知與即時更新的串接式評論。
- `FastCommentsLiveChat` - 基於相同引擎的聊天預設：訊息按時間順序排列，最新訊息在底部，編輯器位於列表下方，具備即時標頭列（連線點 + 使用者數量），透過向上捲動載入無限歷史紀錄，自動捲動至新訊息，且不支援投票或回覆串接。所有預設皆可透過 `config` 覆寫。
- `FastCommentsFeed` - 具備貼文編輯器、媒體、回應、追蹤與即時新貼文橫幅的社交動態。

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### 主題設定

預設外觀是由一組語意化設計代幣 (`FastCommentsTheme`) 產生：顏色、間距、圓角、字體大小、字體粗細與頭像尺寸。透過任意小工具的 `theme` 屬性傳入部分代幣覆寫（型別為 `FastCommentsThemeOverrides`），即可一致地重新樣式化整個樣式樹：

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

深色模式只需切換一組代幣：

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` 屬性仍接受原始的 `IFastCommentsStyles` 樹，以進行精細控制。當同時提供 `theme` 與 `styles` 時，明確的 `styles` 會覆蓋主題樹；若僅提供 `styles`，則會完全取代預設樣式（保持原有行為，因而不影響現有整合與外觀）。`setupDarkModeSkin` 已棄用，建議改用 `theme` 屬性。

### 設定選項

此函式庫旨在支援在 [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) 中定義的所有設定選項，與網頁實作相同。

在此基礎上，React Native 透過 `FastCommentsRNConfig` 增加了幾個 SDK 專屬的選項：

- `hideTopBar` - 隱藏顯示於編輯器上方的已登入使用者 / 通知鈴條。
- `usePressToEdit` - 長按評論以開啟其功能表。
- `disableDownVoting` - 隱藏反向投票按鈕。
- `renderCommentInline` - 在與評論內容相同的 HTML 區塊內呈現評論者資訊。
- `renderLikesToRight` - 將投票/讚區域移至評論右側，而非下方。
- `renderDateBelowComment` - 在評論下方顯示日期。
- `showLiveStatus` - 在評論上方顯示聊天式的「Live」+ 使用者數量標頭列。
- `useInlineSubmitButton` - 在編輯器內以圖示方式呈現送出按鈕。
- `countAboveToggle` - 搭配 `useShowCommentsToggle`，設定在「顯示評論」切換上方顯示的評論數量。
- `preserveFeedScrollPosition` - `FastCommentsFeed` 會在卸載/重新掛載之間保留捲動位置（預設為 true）。

### FastComments 概念

入門時需了解的主要概念為 `tenantId` 與 `urlId`。`tenantId` 為您在 FastComments.com 的帳號識別碼。`urlId` 為評論串所綁定的對象，可為頁面 URL、商品 ID、文章 ID 等。

### 本地化

這些小工具中所有面向使用者的文字（按鈕標籤、佔位文字、空狀態、類似「5 分鐘前」的相對日期、錯誤訊息等）皆為 **伺服器驅動**。元件不會硬編碼英文字串，而是根據請求的語系呈現 FastComments 所提供的翻譯。

若要請求特定語系，請在設定中設定 `locale`：

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

若未設定 `locale`，FastComments 會使用租戶的預設語言。

**編輯文字：** 翻譯由您的 FastComments 控制台管理，而非此 SDK。若要變更文字，可覆寫預設文案或新增語系，於控制台編輯您帳號的翻譯——小工具會自動取得變更，無需重新發布應用程式。此 SDK 不提供英文備援，若在控制台將任意鍵留空，對應語系將顯示空白；請為您支援的每個語系填寫所有鍵值。

### 使用者通知

FastComments 支援[多種情境](https://docs.fastcomments.com/guide-notifications.html)的通知。通知可設定、全域或於單一通知/評論層級取消訂閱，且支援頁面層級的訂閱，讓使用者能訂閱特定頁面或文章的串接。

例如，可使用 Secure SSO 進行使用者驗證，然後定期輪詢未讀通知並推送給使用者。

請參考 [範例 AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) 了解如何取得與翻譯未讀的使用者通知。

### Gif 瀏覽器

預設情況下，未啟用圖片或 gif 選取功能。請參考 [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) 了解如何支援圖片與 gif 上傳。此函式庫提供一個 Gif 瀏覽器，可匿名搜尋與提供圖像，您只需使用它即可。

### 效能

若您發現任何效能問題，請開立含有重現範例與使用裝置資訊的工單。效能是所有 FastComments 函式庫的首要考量。