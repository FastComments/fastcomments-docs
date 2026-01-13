一個 `EmailTemplate` 物件代表租戶的自訂電子郵件範本設定。

系統會透過下列方式選擇要使用的電子郵件範本：

- 其類型識別符，我們稱之為 `emailTemplateId`。這些是常數。
- `domain`。我們會先嘗試尋找與相關物件（例如 `Comment`）所屬網域相符的範本，若找不到相符者，則會嘗試尋找 domain 為 null 或 `*` 的範本。

以下為 `EmailTemplate` 物件的結構：

[inline-code-attrs-start title = '電子郵件範本結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** 唯讀 **/
    createdAt: string
    /** 唯讀 **/
    updatedAt: string
    /** 唯讀 **/
    updatedByUserId: string
    /** 範本應該關聯的網域。 **/
    domain?: string | '*' | null
    /** 以 EJS 語法的電子郵件範本內容。 **/
    ejs: string
    /** 針對每個支援的語系，覆寫翻譯鍵到值的映射。 **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** 表示範本的渲染上下文的物件。 **/
    testData: object
}
[inline-code-end]

### 注意事項

- 您可以從 `/definitions` 端點取得有效的 `emailTemplateId` 值。
- `/definitions` 端點也包含預設翻譯和測試資料。
- 如果結構或測試資料無效，範本將無法儲存。

---